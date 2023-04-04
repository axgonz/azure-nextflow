use crate::app::state::*;
use crate::app::server::*;
use crate::models::api::*;
use crate::services_raw::az_mgmt_containerinstance::*;

use actix_web::{
    get, post,
    web::Data,
    web::Query,
    web::Json,
    Responder,
    HttpResponse
};
use actix_web_grants::proc_macro::{has_permissions};

#[get("/api/nxfutil/dispatch")]
#[has_permissions("admin")]
pub async fn api_dispatch_get() -> impl Responder {
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
    return HttpResponse::Ok()
        .json(help)
}

#[post("/api/nxfutil/dispatch")]
#[has_permissions("admin")]
pub async fn api_dispatch_post(
    req_payload: Json<DispatchRequestPayload>,
    query: Query<QueryParameters>,
    state: Data<AppState>
) -> impl Responder {
    println!("{:#?}", &req_payload);

    println!("[handler] Checking url arguments for `whatif`");
    let what_if: bool = match query.whatif {
        Some(value) => {
            println!("[handler] Found 'whatif' url param {:#?}", value);
            value
        },
        None => {
            false
        }
    };

    println!("[handler] Generating nxfutil command from inputs");
    let nxfutil_cmd = AppServer::generate_nxfutil_cmd(req_payload);

    println!("[handler] Creating nextflow container instance");
    let deployment = AppAzMgmtContainerInstance::create_nxfutil_ci(
        state.identity.clone(),
        &state.variables,
        &nxfutil_cmd,
        what_if
    ).await;

    println!("[handler] Generating ResponsePayload");
    let res_payload = DispatchResponsePayload {
        sub_id: state.variables.nxfutil_az_sub_id.clone(),
        rg_name: state.variables.nxfutil_az_rg_name.clone(),
        ci_name: deployment.0,
        ci_cmd: nxfutil_cmd,
        provisioning_state: deployment.1
    };
    println!("{:#?}", &res_payload);

    return HttpResponse::Ok()
        .json(res_payload)
}
