use crate::app::state::*;
use crate::app::server::*;
use crate::models::api::*;

use actix_web::{
    get, post,
    web::Data,
    web::Json,
    Responder,
    HttpResponse
};

#[get("/api/nxfutil/status")]
pub async fn api_status_get() -> impl Responder {
    let help = StatusRequestPayload {
        summary: true,
        message_count: 1,
        dequeue: true
    };
    return HttpResponse::Ok()
        .json(help)
}

#[post("/api/nxfutil/status")]
pub async fn api_status_post(
    req_payload: Json<StatusRequestPayload>, 
    state: Data<AppState>
) -> impl Responder {
    println!("{:#?}", &req_payload);
    if req_payload.summary {
        return HttpResponse::Ok() 
            .json(serde_json::Value::Array(
                AppServer::get_status_summary(
                    state.identity.clone(), 
                    &state.variables,
                    req_payload.message_count, 
                    req_payload.dequeue
                ).await
            ))                
    }
    else {
        return HttpResponse::Ok() 
            .json(serde_json::Value::Array(
                AppServer::get_status_message(
                    state.identity.clone(), 
                    &state.variables,
                    req_payload.message_count, 
                    req_payload.dequeue
                ).await
            ))
    }
}
