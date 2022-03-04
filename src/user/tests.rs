#[cfg(test)]
mod tests {
    use crate::user::*;
    use actix_web::{
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_user() {
        crate::test::init();

        let request_body = json!({
            "name": "John",
            "handle": "nickname",
            "password": "admin"
        });

        let mut app = test::init_service(App::new().configure(init_routes)).await;

        // create an user(public "register")
        let resp = TestRequest::post()
            .uri("/register")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;
        assert!(resp.status().is_success(), "Failed to register user");

        // Find the created user
        let user: User = test::read_body_json(resp).await;
        let resp = TestRequest::get()
            .uri(&format!("/users/{}", user.id))
            .send_request(&mut app)
            .await;
        assert!(resp.status().is_success(), "Failed to find user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!(user.name, "John", "Found wrong user");
        assert_eq!(user.handle, "nickname", "Found wrong user");
        assert_ne!(user.password, "admin", "Password should be hashed");

        // Check password
        assert!(
            user.verify_password("admin".as_bytes()).unwrap(),
            "Unable to verify password"
        );

        let request_body = json!({
            "name": "John",
            "handle": "newNickname",
        });

        // Update the user
        let resp = TestRequest::put()
            .uri(&format!("/users/{}", user.id))
            .set_json(&request_body)
            .send_request(&mut app)
            .await;
        assert!(resp.status().is_success(), "Failed to update user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!(
            "newNickname", user.handle,
            "Failed to change password for user"
        );

        // Delete the user
        let resp = TestRequest::delete()
            .uri(&format!("/users/{}", user.id))
            .send_request(&mut app)
            .await;
        assert!(resp.status().is_success(), "Failed to delete user");

        // Try find deleted user
        let resp = TestRequest::get()
            .uri(&format!("/users/{}", user.id))
            .send_request(&mut app)
            .await;
        assert!(
            resp.status().is_client_error(),
            "It should not be possible to find the user after deletion"
        );
    }
}
