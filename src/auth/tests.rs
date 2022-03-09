#[cfg(test)]
mod tests {
    use crate::auth::WebToken;
    use crate::routes::routes;
    use crate::user::User;
    use actix_web::{
        http::header,
        test::{self, TestRequest},
        App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_auth() {
        crate::test::init();

        let mut app = test::init_service(App::new().configure(routes)).await;

        let request_body = json!({
            "name": "Auth",
            "handle": "auth",
            "email": "auth@mail.com",
            "password": "password"
        });

        // Register an user
        let res = TestRequest::post()
            .uri("/auth/register")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;

        assert!(res.status().is_success(), "Failed to register");

        // Login
        let res = TestRequest::post()
            .uri("/auth/login")
            .set_json(&request_body)
            .send_request(&mut app)
            .await;

        assert!(res.status().is_success(), "Failed to login");
        let jwt: WebToken = test::read_body_json(res).await;

        // Login with wrong credentials
        let resp = TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "email": "auth@mail.com",
                "password": "wrong"
            }))
            .send_request(&mut app)
            .await;

        assert!(
            resp.status().is_client_error(),
            "Try to login with invalid credentials shouldn't work"
        );

        // Get my profile
        let res = TestRequest::get()
            .uri("/profile")
            .append_header((header::AUTHORIZATION, format!("Bearer {}", jwt.token)))
            .send_request(&mut app)
            .await;
        assert!(res.status().is_success(), "Failed to load loggedIn user");

        let user: User = test::read_body_json(res).await;
        assert_eq!(user.handle, "auth", "Profile fetches wrong user");

        // Get my profile should fail without authorization header
        let res = TestRequest::get()
            .uri("/profile")
            .send_request(&mut app)
            .await;
        assert!(
            res.status().is_client_error(),
            "This routes should be protected"
        );
    }
}
