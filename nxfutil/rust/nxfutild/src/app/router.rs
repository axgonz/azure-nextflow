use axum::{
    Router,
    routing::{get, post},
    response::IntoResponse, 
    http::StatusCode,
    Json
};

use serde_json::{
    Value
};

#[derive(Debug)]
struct AppRouter {
    app_router: Router
}

impl AppRouter {   
    async fn new() -> Self {
        let func_name = env::var("FUNCTIONS_FUNCTION_NAME").unwrap();
        let api_root = format!("api/{}", func_name);

        // Initialize any app state or services etc
        let app_server = AppServer::new();
        AppServer::init_services(&app_server).await;

        // https://docs.rs/axum/latest/axum/
        AppRouter {
            app_router: Router::new()
                .route(
                    format!("/{}", api_root).as_str(), 
                    get(Self::api_root_get)
                )
                .route(
                    format!("/{}", api_root).as_str(), 
                    post({
                        let app_server = app_server.clone();
                        move |body| Self::api_root_post(body, app_server)
                    })
                )
        }
    }
    async fn api_root_get() -> impl IntoResponse {
        return StatusCode::OK
    }
    async fn api_root_post(Json(req_payload): Json<Value>, app_server: AppServer) -> impl IntoResponse {
        AppServer::send_message_to_queue(Json(req_payload), app_server).await;
        return StatusCode::OK
    }
}