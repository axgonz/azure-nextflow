mod app; 
mod routes;
mod models;
mod services;

use az_app_identity::*;

use app::{
    variables::*,
    state::*,
};

use services::{
    az_storage_queues::*,
};

use routes::{
    root::*,
};

use actix_web::{
    web::Data,
    App, 
    HttpServer
};

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

    let queue = AppAzStorageQueue::new("nextflow", app_identity.clone(), &app_variables);
    queue.init().await;
    
    let app_state = AppState {
        identity: app_identity,
        variables: app_variables,
        queue: queue,
    };

    let state = Data::new(app_state);

    println!("\n[handler] Listening on http://{}:{}\n", addr, port);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(api_root_get)
            .service(api_root_post)
    })
    .bind((addr, port))?
    .run()
    .await
}