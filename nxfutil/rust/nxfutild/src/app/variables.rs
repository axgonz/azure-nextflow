//# AppVariables
#[derive(Debug, Clone)]
pub struct AppVariables {
    pub kv_name: String,
    pub fc_name: String
}

impl AppVariables {
    pub fn new() -> Self {
        Self {
            kv_name: "".to_string(),
            fc_name: "".to_string()
        }
    }
    pub fn init(variables: &mut AppVariables) {
        variables.kv_name = Self::variable("AZURE_KEYVAULT_NAME");
        variables.fc_name = Self::variable("FUNCTIONS_FUNCTION_NAME");
    }
    pub fn variable(name: &str) -> String {
        match env::var(name) {
            Ok(value) => return value,
            Err(_) => {
                panic!("Unable to load environment variable: {:#?}", name)
            }
        }
    }
}
