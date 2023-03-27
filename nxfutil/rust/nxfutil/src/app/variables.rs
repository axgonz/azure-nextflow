//# AppVariables
#[derive(Debug, Clone)]
pub struct AppVariables {
    pub kv_name: String,
    pub fn_name: String,
    pub ci_name: String
}

impl AppVariables {
    pub fn new() -> Self {
        Self {
            kv_name: "".to_string(),
            fn_name: "".to_string(),
            ci_name: "".to_string()
        }
    }
    pub fn init(variables: &mut AppVariables) {
        variables.kv_name = Self::variable("AZURE_KEYVAULT_NAME");
        variables.fn_name = Self::variable("NXFUTIL_API_FQDN");
        variables.ci_name = Self::variable("NXFUTIL_DISPATCHER");
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
