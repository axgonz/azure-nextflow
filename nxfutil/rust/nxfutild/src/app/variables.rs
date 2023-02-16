//# AppVariables
#[derive(Debug, Clone)]
pub struct AppVariables {
    pub st_name: String,
    pub func_name: String,
}

impl Default for AppVariables {
    fn default() -> Self {
        Self {
            st_name: env::var("AZURE_STORAGEACCOUNT_NAME").unwrap(),
            func_name: env::var("FUNCTIONS_FUNCTION_NAME").unwrap()
        }
    }
}

impl AppVariables {
    pub fn new() -> Self {
        Self::default()
    }
}
