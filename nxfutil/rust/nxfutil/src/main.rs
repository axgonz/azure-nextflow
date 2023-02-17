include!("app/server.rs");
include!("app/variables.rs");
include!("app/secrets.rs");
include!("services/az-identity.rs");
include!("services/az-security-keyvault.rs");

use std::{
    io,
    io::Write,
    env
};

#[tokio::main]
async fn main() {
    let az_identity = AppAzIdentity::new();

    let mut variables = AppVariables::new();
    AppVariables::init(&mut variables);

    let mut secrets = AppSecrets::new(az_identity.credential.clone(), &variables);
    AppSecrets::init(&mut secrets).await;

    let mut server: AppServer = AppServer::new(variables, secrets, az_identity);
}