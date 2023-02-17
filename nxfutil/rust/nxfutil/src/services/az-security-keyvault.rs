use azure_security_keyvault::{
    SecretClient
};

#[derive(Debug, Clone)]
pub struct AppAzSecurityKeyVault {
    pub secret_client: SecretClient
}

impl AppAzSecurityKeyVault { 
    fn new(credential: Arc<DefaultAzureCredential>, variables: &AppVariables) -> Self {
        let kv_url = format!("https://{}.vault.azure.net", variables.kv_name);   
        Self {
            secret_client: SecretClient::new(&kv_url, credential).unwrap()
        }
    }
}