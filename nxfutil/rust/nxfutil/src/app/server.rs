#[derive(Clone)]
pub struct AppServer {
    pub variables: AppVariables,
    pub secrets: AppSecrets,
    pub az_identity: AppAzIdentity
}

impl AppServer {
    fn new(variables: AppVariables, secrets: AppSecrets, az_identity: AppAzIdentity) -> Self {
        Self {
            az_identity: az_identity,
            variables: variables,
            secrets: secrets            
        }
    }
    async fn init(server: &AppServer) {
    }
}