#[cfg(test)]
mod tests {
    use crate::response::Response;
    use crate::tweet::{init_routes, Tweet};
    use actix_web::{
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_user() {
        // prepare
        crate::test::init();
        let message_test: &str = "Hey, I'm a test tweet!";
        let request_body = json!({ "message": message_test });
        let mut app = test::init_service(App::new().configure(init_routes)).await;

        // create a tweet
        let res = TestRequest::post()
            .uri("/tweets")
            .set_json(&request_body)
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
