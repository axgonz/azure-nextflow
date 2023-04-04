use actix_web::{
    get,
    Responder,
    HttpResponse
};
use actix_web_grants::proc_macro::{has_permissions};

#[get("/api/nxfutil")]
#[has_permissions("admin")]
pub async fn api_root_get() -> impl Responder {
    return HttpResponse::Ok()
        .body("Hello World!\n\nGET api/nxfutil/dispatch\nGET api/nxfutil/status\nGET api/nxfutil/terminate")
}