//# AppVariables
#[derive(Debug, Clone)]
pub struct AppVariables {
    pub st_name: String,
    pub q_name: String,
    pub sub_id: String,
    pub rg_name: String,
    pub kv_name: String,
    pub cr_server: String,
    pub cr_name: String,
    pub fn_name: String,
    pub msi_name: String,
    pub msi_client_id: String,
}

impl AppVariables {
    pub fn new() -> Self {
        Self {
            st_name: "".to_string(),
            q_name: "nextflow".to_string(),
            sub_id: "".to_string(),
            rg_name: "".to_string(),
            kv_name: "".to_string(),
            cr_server: "".to_string(),
            cr_name: "".to_string(),
            fn_name: "".to_string(),
            msi_name: "".to_string(),
            msi_client_id: "".to_string()
        }
    }
    pub fn init(variables: &mut AppVariables) {
        variables.st_name = Self::get("NXFUTIL_AZ_ST_NAME");
        variables.sub_id = Self::get("NXFUTIL_AZ_SUB_ID");
        variables.rg_name = Self::get("NXFUTIL_AZ_RG_NAME");
        variables.kv_name = Self::get("NXFUTIL_AZ_KV_NAME");
        variables.cr_server = Self::get("NXFUTIL_AZ_CR_NAME");
        variables.cr_name = Self::get("NXFUTIL_AZ_CR_NAME");
        variables.fn_name = Self::get("NXFUTIL_AZ_FA_NAME");
        variables.msi_name = Self::get("NXFUTIL_AZ_MSI_NAME");
        variables.msi_client_id = Self::get("NXFUTIL_AZ_MSI_ID");
    }
    pub fn get(name: &str) -> String {
        match env::var(name) {
            Ok(value) => return value,
            Err(_) => {
                panic!("Unable to load environment variable: {:#?}", name)
            }
        }
    }
}
