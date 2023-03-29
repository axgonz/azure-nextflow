use az_app_identity::*;
pub use az_app_secrets::*;

#[derive(AzAppSecretsNew, AzAppSecretsInit, Debug)]
pub struct AppSecrets {
}