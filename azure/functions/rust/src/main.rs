include!("app/app.rs");
include!("app/router.rs");
include!("app/server.rs");
include!("app/variables.rs");
include!("services/az-identity.rs");
include!("services/az-storage-queues.rs");
include!("services-raw/az-mgmt-containerinstance.rs");

use axum::{
    Server
};

use std::{
    env,
    net::SocketAddr
};

// Boiler-plate main fn: http server for Azure Functions
#[tokio::main]
async fn main() {
    // Grab the port from the azure functions runtime; or use port 3000.
    let port: u16 = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    // Define the function address, this will be a binary on azure functions listen on localhost.
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Define our service with routes, any shared state and/or middleware (aka. exceptions), etc.
    let app = AppRouter::new().await.app_router;

    // Log that everything is okay and we are ready to listen
    println!("\n[handler] Listening on {:#?}\n", &addr);
    
    // Start listening and panic if anything doesn't work.
    Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}