//# AppSecrets
#[derive(Debug, Clone)]
pub struct AppSecrets {
    pub az_security_keyvault: AppAzSecurityKeyVault,
    // pub secret_name: String
}

impl AppSecrets {
    pub fn new(credential: Arc<DefaultAzureCredential>, variables: &AppVariables) -> Self {
        Self {
            az_security_keyvault: AppAzSecurityKeyVault::new(credential, variables),
            // secret_name: "".to_string()
        }
    }
    pub async fn init(secrets: &mut AppSecrets) {
        // secrets.secret_name = Self::secret("my-secret-name", secrets).await;
    }
    pub async fn secret(name: &str, secrets: &AppSecrets) -> String {
        match secrets.az_security_keyvault.secret_client.get(name).await {
            Ok(value) => {
                println!("[az-security-keyvault] Requesting key vault secret {:#?}...Ok", name);
                return value.value
            }
            Err(error) => {
                println!("[az-security-keyvault] Requesting key vault secret {:#?}...Err", name);
                panic!("{}", error)
            }
        }
    }
}