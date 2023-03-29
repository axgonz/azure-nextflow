mod app; 
mod routes;
mod models;
mod services;
mod services_raw;

use az_app_identity::*;
use app::{
    variables::*,
    state::*,
};
use routes::{
    root::*,
    dispatch::*,
    status::*,
    terminate::*,
};

use actix_web::{
    web::Data,
    App, 
    HttpServer
};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr: String = match std::env::var("ACTIXWEB_SERVE_ADDRESS") {
        Ok(value) => value,
        Err(_) => "0.0.0.0".to_string()
    };

    let port: u16 = match std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(value) => value.parse().unwrap(),
        Err(_) => "3000".parse().unwrap()
    };

    let app_identity = AppIdentity::new();

    let mut app_variables = AppVariables::new();
    AppVariables::init(&mut app_variables);
    
    let app_state = AppState {
        identity: app_identity,
        variables: app_variables
    };

    let state = Data::new(app_state);

    println!("[handler] CORS are set to Cors::permissive()");
    println!("\n[handler] Listening on http://{}:{}\n", addr, port);
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .service(api_root_get)
            .service(api_dispatch_get)
            .service(api_dispatch_post)
            .service(api_status_get)
            .service(api_status_post)
            .service(api_terminate_get)
            .service(api_terminate_post)
    })
    .bind((addr, port))?
    .run()
    .await
}