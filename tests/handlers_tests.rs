#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    // use crate::handlers; if test file same /src 
    // use demo_rest_api::user_handlers;
    use demo_api::handlers::user_handler;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(
            App::new().route("/", web::get().to(user_handler::hello))
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_users() {
        let app = test::init_service(
            App::new().route("/users_local", web::get().to(user_handler::get_users))
        ).await;

        let req = test::TestRequest::get().uri("/users_local").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_user_not_found() {
        let app = test::init_service(
            App::new().route("/users_local/{id}", web::get().to(user_handler::get_user))
        ).await;

        let req = test::TestRequest::get().uri("/users_local/999").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 404);
    }

    #[actix_web::test]
    async fn test_create_user_missing_name() {
        let app = test::init_service(
            App::new().route("/users_local", web::post().to(user_handler::create_user))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users_local")
            .set_json(serde_json::json!({ "email": "test@example.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_create_user_invalid_email() {
        let app = test::init_service(
            App::new().route("/users_local", web::post().to(user_handler::create_user))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users_local")
            .set_json(serde_json::json!({ "name": "Test", "email": "not-an-email" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }

    #[actix_web::test]
    async fn test_create_user_success() {
        let app = test::init_service(
            App::new().route("/users_local", web::post().to(user_handler::create_user))
        ).await;

        let req = test::TestRequest::post()
            .uri("/users_local")
            .set_json(serde_json::json!({ "name": "Alice", "email": "alice@test.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }
}