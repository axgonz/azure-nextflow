//# AppSecrets
#[derive(Debug, Clone)]
pub struct AppSecrets {
    az_security_keyvault: AppAzSecurityKeyVault,
    pub st_name: String,
    pub cr_server: String,
    pub cr_username: String,
    pub cr_password: String
}

impl AppSecrets {
    pub fn new(credential: Arc<DefaultAzureCredential>, variables: &AppVariables) -> Self {
        Self {
            az_security_keyvault: AppAzSecurityKeyVault::new(credential, variables),
            st_name: "".to_string(),
            cr_server: "".to_string(),
            cr_username: "".to_string(),
            cr_password: "".to_string()
        }
    }
    pub async fn init(secrets: &mut AppSecrets) {
        secrets.st_name = Self::get("azure-storage-accountName", secrets).await;
        secrets.cr_server = Self::get("azure-registry-server", secrets).await;
        secrets.cr_username = Self::get("azure-registry-username", secrets).await;
        secrets.cr_password = Self::get("azure-registry-password", secrets).await;
    }
    pub async fn get(name: &str, secrets: &AppSecrets) -> String {
        match secrets.az_security_keyvault.secret_client.get(name).await {
            Ok(value) => {
                println!("[handler][az-security-keyvault] Requesting key vault secret {:#?}...Ok", name);
                return value.value
            }
            Err(error) => {
                println!("[handler][az-security-keyvault] Requesting key vault secret {:#?}...Err", name);
                println!("{:#?}", error);
                panic!("{}", error)
            }
        }
    }
}