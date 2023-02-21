use axum::{
    Router,
    routing::{get, post},
    extract::Query,
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
                    "/api/nxfutil_status",
                    get({
                        let server = server.clone();
                        move || Self::api_rootd_get(server)
                    })
                )       
        }
    }
    async fn api_root_get() -> impl IntoResponse {
        return (StatusCode::OK, "Hello World again!")
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
    
        println!("[handler] Deploying nextflow container instance");
        let deployment = AppAzMgmtContainerInstance::deploy_nxfutil_ci(
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
    async fn api_rootd_get(server: AppServer) -> impl IntoResponse {
        let response = App::generate_status_update(&server).await;
        return (StatusCode::OK, Json(response))
    }    
}