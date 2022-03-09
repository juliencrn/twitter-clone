#[cfg(test)]
mod tests {
    use crate::auth::WebToken;
    use crate::response::Response;
    use crate::routes::routes;
    use crate::tweet::Tweet;
    use crate::user::User;
    use actix_web::{
        http::header,
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;

    fn register() -> TestRequest {
        TestRequest::post().uri("/auth/register").set_json(json!({
            "name": "tweet",
            "handle": "tweet",
            "email": "tweet@mail.com",
            "password": "password"
        }))
    }

    fn login() -> TestRequest {
        TestRequest::post().uri("/auth/login").set_json(json!({
            "email": "tweet@mail.com",
            "password": "password"
        }))
    }

    #[actix_web::test]
    async fn test_tweet() {
        // prepare
        crate::test::init();
        let message_test: &str = "Hey, I'm a test tweet!";
        let mut app = test::init_service(App::new().configure(routes)).await;

        // Register and Login to exec protected routes
        let res = register().send_request(&mut app).await;
        assert!(res.status().is_success(), "Failed to register");

        // Login
        let res = login().send_request(&mut app).await;
        assert!(res.status().is_success(), "Failed to login");

        // Extract Authorization header
        let WebToken { token } = test::read_body_json(res).await;
        let auth_headers = (header::AUTHORIZATION, format!("Bearer {}", token));

        // /profile to get user id
        let res = TestRequest::get()
            .uri("/profile")
            .append_header(auth_headers.clone())
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to get logged-in user");

        let user: User = test::read_body_json(res).await;

        // create a tweet
        let res = TestRequest::post()
            .uri("/tweets")
            .set_json(json!({ "message": message_test }))
            .append_header(auth_headers.clone())
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to create tweet");
        let tweet: Tweet = test::read_body_json(res).await;

        println!("tweet: {:#?}", tweet);
        println!("/tweets/{}", tweet.id.to_string());

        // find a tweet
        let res = TestRequest::get()
            .uri(&format!("/tweets/{}", tweet.id.to_string()))
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to find tweet");
        let tweet: Tweet = test::read_body_json(res).await;
        assert_eq!(tweet.message, message_test, "Found wrong tweet");
        assert_eq!(tweet.likes, 0, "Wrong default like count");

        // find tweets by user
        let res = TestRequest::get()
            .uri(&format!("/tweets?user_id={}", user.id))
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to find tweets by user");
        let tweets: Response<Tweet> = test::read_body_json(res).await;
        let tweet_count = tweets.results.len();
        assert_eq!(tweet_count, 1, "Wrong tweets by user count");
        let tweet = tweets.results.get(0).unwrap();
        assert_eq!(tweet.message, message_test, "Found wrong tweet");
        assert_eq!(tweet.author, user.id, "Found wrong tweet");
        assert_eq!(tweet.likes, 0, "Wrong default like count");

        // find all tweets
        let res = TestRequest::get()
            .uri("/tweets")
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to find all tweets");

        let tweet_list: Response<Tweet> = test::read_body_json(res).await;
        assert_eq!(
            tweet_list.results[0].message, message_test,
            "Wrong tweet count"
        );

        // delete tweet
        let res = TestRequest::delete()
            .uri(&format!("/tweets/{}", tweet.id.to_string()))
            .append_header(auth_headers.clone())
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to delete tweet");

        // try get deleted tweet
        let res = TestRequest::get()
            .uri(&format!("/tweets/{}", tweet.id.to_string()))
            .send_request(&mut app)
            .await;
        assert!(
            res.status().is_client_error(),
            "This tweet should be deleted"
        );

        // Test default request
        let res = TestRequest::default().send_request(&mut app).await;
        assert!(res.status().is_client_error(), "Should be not allowed");
    }
}
