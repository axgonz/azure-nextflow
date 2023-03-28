use crate::app::{
    state::*,
    server::*,
};

use actix_web::{
    get, post,
    web::Json,
    web::Data,
    Responder,
    HttpResponse
};

use serde_json::Value;

#[get("/api/nxfutild")]
pub async fn api_root_get() -> impl Responder {
    return HttpResponse::Ok()
}

#[post("/api/nxfutild")]
pub async fn api_root_post(
    req_payload: Json<Value>, 
    state: Data<AppState>
) -> impl Responder {
    AppServer::send_status_message(req_payload, &state.queue).await;
    return HttpResponse::Ok()
}
