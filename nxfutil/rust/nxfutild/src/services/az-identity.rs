use std::{
    sync::Arc
};
use azure_identity::{
    DefaultAzureCredential, 
    DefaultAzureCredentialEnum, 
    AzureCliCredential, 
    ImdsManagedIdentityCredential
};

#[derive(Clone)]
pub struct AppAzIdentity {
    pub credential: Arc<DefaultAzureCredential>
}

impl AppAzIdentity {
    fn new() -> Self {
        AppAzIdentity {
            credential: Self::azure_login()
        }
    }
    fn azure_login() -> Arc<DefaultAzureCredential> {
        /* Build Azure credential
            If using a User Assigned Managed Identity you must set the `AZURE_CLIENT_ID`
            environment variable to give `DefaultAzureCredential::default()` the right context. 
            
            This (`AZURE_CLIENT_ID`) can be omitted if using a System Assigned Managed Identity 
            and when developing locally. 
            When developing locally make sure to run `az login` somewhere on your local PC to cache
            your own credential for the builder to use; do not provide `AZURE_CLIENT_ID`.
        */  
        let credential_sources = match env::var("AZURE_CLIENT_ID") {
            Ok(azure_client_id) => {
                AppServer::log("AZURE_CLIENT_ID is set, will try to use User Assigned Managed Identity".to_string());
                vec![
                    DefaultAzureCredentialEnum::ManagedIdentity(
                        ImdsManagedIdentityCredential::default()
                            .with_client_id(azure_client_id)
                    ),
                    DefaultAzureCredentialEnum::AzureCli(
                        AzureCliCredential::new()
                    )
                ]
            },
            Err(_) => {
                AppServer::log("AZURE_CLIENT_ID is unset, will try to use System Assigned Managed Identity".to_string());
                vec![
                    DefaultAzureCredentialEnum::ManagedIdentity(
                        ImdsManagedIdentityCredential::default()
                    ),
                    DefaultAzureCredentialEnum::AzureCli(
                        AzureCliCredential::new()
                    )
                ]
            }
        };    
    
        return Arc::new(DefaultAzureCredential::with_sources(credential_sources));
    }
}
