use axum::{
    Router,
    routing::{get, post},
    extract::Query,
    response::IntoResponse, 
    http::StatusCode,
    Json
};

use tower_http::cors::{
    Any, 
    CorsLayer,
};

use http::{
    Request, 
    Response, 
    Method, 
    header
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
        println!("[handler] CORS are set to CorsLayer::very_permissive()");
        
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
                    "/api/nxfutil/dispatch", 
                    get(Self::api_dispatch_get)
                )
                .route(
                    "/api/nxfutil/dispatch", 
                    post({
                        let server = server.clone();
                        move |query, body| Self::api_dispatch_post(query, body, server)
                    })
                )
                .route(
                    "/api/nxfutil/terminate", 
                    get(Self::api_terminate_get)
                )
                .route(
                    "/api/nxfutil/terminate", 
                    post({
                        let server = server.clone();
                        move |query, body| Self::api_terminate_post(query, body, server)
                    })
                )
                .route(
                    "/api/nxfutil/status",
                    get(Self::api_status_get)
                )
                .route(
                    "/api/nxfutil/status", 
                    post({
                        let server = server.clone();
                        move |body| Self::api_status_post(body, server)
                    })
                )
                .layer(CorsLayer::very_permissive())
        }
    }
    async fn api_root_get() -> impl IntoResponse {
        return (StatusCode::OK, "Hello World!\n\nGET api/nxfutil/dispatch\nGET api/nxfutil/terminate\nGET api/nxfutil/status")
    }
    async fn api_dispatch_get() -> impl IntoResponse {
        let help = DispatchRequestPayload {
            config_uri: "String".to_string(),
            pipeline_uri: "String".to_string(),
            parameters_uri: "String".to_string(),
            parameters_json: Some(vec![NextflowParam {
                name: "String".to_string(),
                value: "Value".into(),
            }]),
            auto_delete: true
        };
        return (StatusCode::OK, Json(help))
    }    
    async fn api_dispatch_post(
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
    async fn api_terminate_get() -> impl IntoResponse {
        let help = TerminateRequestPayload {
            ci_name: "String".to_string()
        };
        return (StatusCode::OK, Json(help))
    }        
    async fn api_terminate_post(
        Query(url_params): Query<HashMap<String, String>>,
        Json(req_payload): Json<TerminateRequestPayload>, 
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

        println!("[handler] Deleting nextflow container instance");
        let deployment = AppAzMgmtContainerInstance::delete_nxfutil_ci(
            server.az_identity.credential.clone(), 
            &server.variables, 
            &req_payload.ci_name,
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
    async fn api_status_get() -> impl IntoResponse {
        let help = StatusRequestPayload {
            summary: true,
            message_count: 1,
            dequeue: true
        };
        return (StatusCode::OK, Json(help))
    }
    async fn api_status_post(
        req_payload: Json<StatusRequestPayload>, 
        server: AppServer
    ) -> impl IntoResponse {
        println!("{:#?}", &req_payload);
        if req_payload.summary {
            return (StatusCode::OK, 
                Json(serde_json::Value::Array(
                    App::generate_status_update(req_payload.message_count, req_payload.dequeue, &server).await
                ))                
            );
        }
        if req_payload.dequeue {
            return (StatusCode::OK, 
                Json(serde_json::Value::Array(
                    AppServer::get_message_from_queue(req_payload.message_count, &server).await
                ))
            );
        }
        else {
            return (StatusCode::OK, 
                Json(serde_json::Value::Array(
                    AppServer::peak_message_from_queue(req_payload.message_count, &server).await
                ))
            );
        }
    }      
}