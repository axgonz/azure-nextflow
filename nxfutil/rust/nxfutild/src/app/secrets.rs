//# AppSecrets
#[derive(Debug, Clone)]
pub struct AppSecrets {
    pub az_security_keyvault: AppAzSecurityKeyVault,
    pub st_name: String
}

impl AppSecrets {
    pub fn new(credential: Arc<DefaultAzureCredential>, variables: &AppVariables) -> Self {
        Self {
            az_security_keyvault: AppAzSecurityKeyVault::new(credential, variables),
            st_name: "".to_string()
        }
    }
    pub async fn init(secrets: &mut AppSecrets) {
        secrets.st_name = Self::secret("azure-storage-accountName", secrets).await;
    }
    pub async fn secret(name: &str, secrets: &AppSecrets) -> String {
        print!("[az-security-keyvault] Requesting key vault secret {:#?}...", name);
        io::stdout().flush().unwrap();
        match secrets.az_security_keyvault.secret_client.get(name).await {
            Ok(value) => {
                println!("Ok");
                return value.value
            }
            Err(error) => {
                println!("Err");
                panic!("{}", error)
            }
        }
    }
}