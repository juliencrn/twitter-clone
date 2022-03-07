#[cfg(test)]
mod tests {
    use crate::auth::WebToken;
    use crate::response::Response;
    use crate::routes::routes;
    use crate::tweet::Tweet;
    use actix_web::{
        http::header,
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_tweet() {
        // prepare
        crate::test::init();
        let message_test: &str = "Hey, I'm a test tweet!";
        let mut app = test::init_service(App::new().configure(routes)).await;

        // Register and Login to exec protected routes
        let res = TestRequest::post()
            .uri("/auth/register")
            .set_json(json!({
                "name": "tweet",
                "handle": "tweet",
                "password": "password"
            }))
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to register");

        let res = TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "handle": "tweet",
                "password": "password"
            }))
            .send_request(&mut app)
            .await;

        assert!(res.status().is_success(), "Failed to login");
        let WebToken { token } = test::read_body_json(res).await;
        let auth_headers = (header::AUTHORIZATION, format!("Bearer {}", token));

        // create a tweet
        let res = TestRequest::post()
            .uri("/tweets")
            .set_json(json!({ "message": message_test }))
            .append_header(auth_headers.clone())
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to create tweet");
        let tweet: Tweet = test::read_body_json(res).await;

        // find a tweet
        let res = TestRequest::get()
            .uri(&format!("/tweets/{}", tweet.id.to_string()))
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to find tweet");

        let tweet: Tweet = test::read_body_json(res).await;
        assert_eq!(tweet.message, message_test, "Found wrong tweet");
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
