use crate::app::state::*;
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

#[get("/api/nxfutil/terminate")]
#[has_permissions("admin")]
pub async fn api_terminate_get() -> impl Responder {
    let help = TerminateRequestPayload {
        ci_name: "String".to_string()
    };
    return HttpResponse::Ok()
        .json(help)
}

#[post("/api/nxfutil/terminate")]
#[has_permissions("admin")]
pub async fn api_terminate_post(
    req_payload: Json<TerminateRequestPayload>,
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

    println!("[handler] Deleting nextflow container instance");
    let deployment = AppAzMgmtContainerInstance::delete_nxfutil_ci(
        state.identity.clone(),
        &state.variables,
        &req_payload.ci_name,
        what_if
    ).await;

    println!("[handler] Generating ResponsePayload");
    let res_payload = TerminateResponsePayload {
        sub_id: state.variables.nxfutil_az_sub_id.clone(),
        rg_name: state.variables.nxfutil_az_rg_name.clone(),
        ci_name: deployment.0,
        provisioning_state: deployment.1
    };
    println!("{:#?}", &res_payload);

    return HttpResponse::Ok()
        .json(res_payload)
}
