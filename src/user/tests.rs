#[cfg(test)]
mod tests {
    use crate::auth::WebToken;
    use crate::routes;
    use crate::user::User;
    use actix_web::{
        http::header,
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_user() {
        crate::test::init();

        let request_body = json!({
            "name": "User",
            "handle": "user",
            "email": "user@mail.com",
            "password": "password"
        });

        let mut app = test::init_service(App::new().configure(routes::routes)).await;

        // create an user(aka "register")
        let resp = TestRequest::post()
            .uri("/auth/register")
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
        assert_eq!(user.name, "User", "Found wrong user");
        assert_eq!(user.handle, "user", "Found wrong user");

        // Login just to get the auth token
        let res = TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;

        assert!(res.status().is_success(), "Failed to login");
        let WebToken { token } = test::read_body_json(res).await;

        println!("Token: {}", token);

        let request_body = json!({
            "name": "Johnny",
            "handle": "newNickname",
        });
        let auth_headers = (header::AUTHORIZATION, format!("Bearer {}", token));

        // Update the user
        let resp = TestRequest::put()
            .uri(&format!("/users/{}", user.id))
            .set_json(&request_body)
            .append_header(auth_headers.clone())
            .send_request(&mut app)
            .await;
        assert!(resp.status().is_success(), "Failed to update user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!("Johnny", user.name, "Failed to update the name");
        assert_eq!("newNickname", user.handle, "Failed to update the handle");

        // Delete the user
        let resp = TestRequest::delete()
            .uri(&format!("/users/{}", user.id))
            .append_header(auth_headers.clone())
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
