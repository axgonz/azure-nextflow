use axum::{
    Router,
    routing::{get, post, delete},
    extract::Query,
    extract::Path,
    response::IntoResponse, 
    http::StatusCode,
    Json
};

use serde::{
    Deserialize, 
    Serialize
};

use serde_json::{
    Value,
};

use std::{
    collections::HashMap
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

        // https://docs.rs/axum/latest/axum/
        AppRouter {
            app_router: Router::new()
                .route(
                    "/api/nxfutil", 
                    get(Self::api_root_get)
                )
                .route(
                    "/api/nxfutil", 
                    post({
                        let server = server.clone();
                        move |query, body| Self::api_root_post(query, body, server)
                    })
                )
                .route(
                    "/api/nxfutil/:ci_name", 
                    delete({
                        let server = server.clone();
                        move |query, path| Self::api_root_delete(query, path, server)
                    })
                )                    
                .route(
                    "/api/nxfutil_status",
                    get({
                        let server = server.clone();
                        move || Self::api_status_get(server)
                    })
                ) 
                .route(
                    "/api/nxfutil_status", 
                    post({
                        let server = server.clone();
                        move |body| Self::api_status_post(body, server)
                    })
                )                      
        }
    }
    async fn api_root_get() -> impl IntoResponse {
        return (StatusCode::OK, "Hello World!")
    }
    async fn api_root_post(
        Query(url_params): Query<HashMap<String, String>>,
        Json(req_payload): Json<DispatchRequestPayload>, 
        server: AppServer
    ) -> impl IntoResponse {
        println!("{:#?}", &req_payload);

        println!("[handler] Checking url arguments for `whatif`");
        let what_if: bool = match url_params.get("whatif") {
            Some(key_value) => {
                println!("[handler] Found 'whatif' url param {:#?}", key_value);
                if key_value.to_lowercase() == "true" {
                    true
                }
                else {
                    false
                }
            },
            None => {
                false
            }
        };

        println!("[handler] Generating nxfutil command from inputs");
        let nxfutil_cmd = App::generate_nxfutil_cmd(req_payload, url_params);
    
        println!("[handler] Creating nextflow container instance");
        let deployment = AppAzMgmtContainerInstance::create_nxfutil_ci(
            server.az_identity.credential.clone(), 
            &server.variables, 
            &server.secrets, 
            &nxfutil_cmd, 
            what_if
        ).await;

        println!("[handler] Generating ResponsePayload");
        let res_payload = DispatchResponsePayload { 
            sub_id: server.variables.sub_id,
            rg_name: server.variables.rg_name,
            ci_name: deployment.0,
            ci_cmd: nxfutil_cmd,
            provisioning_state: deployment.1
        };
        println!("{:#?}", &res_payload);

        return (StatusCode::OK, Json(res_payload))
    }
    async fn api_root_delete(
        Query(url_params): Query<HashMap<String, String>>,
        Path(ci_name): Path<String>,
        server: AppServer
    ) -> impl IntoResponse {
        println!("{:#?}", &ci_name);

        println!("[handler] Checking url arguments for `whatif`");
        let what_if: bool = match url_params.get("whatif") {
            Some(key_value) => {
                println!("[handler] Found 'whatif' url param {:#?}", key_value);
                if key_value.to_lowercase() == "true" {
                    true
                }
                else {
                    false
                }
            },
            None => {
                false
            }
        };

        println!("[handler] Deleting nextflow container instance");
        let deployment = AppAzMgmtContainerInstance::delete_nxfutil_ci(
            server.az_identity.credential.clone(), 
            &server.variables, 
            &ci_name,
            what_if
        ).await;

        println!("[handler] Generating ResponsePayload");
        let res_payload = TerminateResponsePayload { 
            sub_id: server.variables.sub_id,
            rg_name: server.variables.rg_name,
            ci_name: deployment.0,
            provisioning_state: deployment.1
        };
        println!("{:#?}", &res_payload);

        return (StatusCode::OK, Json(res_payload))
    }    
    async fn api_status_get(
        server: AppServer
    ) -> impl IntoResponse {
        return (StatusCode::OK, Json(App::generate_status_update(&server).await))
    }
    async fn api_status_post(
        req_payload: Json<StatusRequestPayload>, 
        server: AppServer
    ) -> impl IntoResponse {
        let mut messages: Vec<Value> = vec![];
        if req_payload.dequeue {
            messages = AppServer::get_message_from_queue(req_payload.message_count, &server).await
        }
        else {
            messages = AppServer::peak_message_from_queue(req_payload.message_count, &server).await
        }
        return (StatusCode::OK, Json(serde_json::Value::Array(messages)))
    }      
}