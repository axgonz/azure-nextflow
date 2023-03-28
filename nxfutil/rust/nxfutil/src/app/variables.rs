pub use az_app_variables::*;

#[derive(AzAppVariablesNew, AzAppVariablesInit, Debug)]
pub struct AppVariables {
    pub nxfutil_dispatcher: String,
    pub nxfutil_api_fqdn: String,
    pub nxfutil_az_st_name: String,
    pub nxfutil_az_kv_name: String,
}
