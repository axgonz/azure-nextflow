use actix_web::{
    get,
    Responder,
    HttpResponse
};

#[get("/api/nxfutil")]
pub async fn api_root_get() -> impl Responder {
    return HttpResponse::Ok()
        .body("Hello World!\n\nGET api/nxfutil/dispatch\nGET api/nxfutil/status\nGET api/nxfutil/terminate")
}