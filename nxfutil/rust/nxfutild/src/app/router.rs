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
        let az_identity = AppAzIdentity::new();

        let mut variables = AppVariables::new();
        AppVariables::init(&mut variables);
    
        let mut secrets = AppSecrets::new(az_identity.credential.clone(), &variables);
        AppSecrets::init(&mut secrets).await;
    
        let server: AppServer = AppServer::new(variables, secrets, az_identity);
        AppServer::init(&server).await;

        let api_root = format!("api/{}", server.variables.fc_name);

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
                        move |body| Self::api_root_post(body, server.clone())
                    })
                )
        }
    }
    async fn api_root_get() -> impl IntoResponse {
        return StatusCode::OK
    }
    async fn api_root_post(Json(req_payload): Json<Value>, server: AppServer) -> impl IntoResponse {
        AppServer::send_message_to_queue(Json(req_payload), &server).await;
        return StatusCode::OK
    }
}