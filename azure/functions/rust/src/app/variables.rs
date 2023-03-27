pub use az_app_variables::*;

#[derive(AzAppVariablesNew, AzAppVariablesInit, Debug)]
pub struct AppVariables {
    pub nxfutil_api_fqdn: String,
    pub nxfutil_az_sub_id: String,
    pub nxfutil_az_rg_name: String,
    pub nxfutil_az_st_name: String,
    pub nxfutil_az_cr_name: String,
    pub nxfutil_az_kv_name: String,
    pub nxfutil_az_msi_name: String,
    pub nxfutil_az_msi_id: String,
}
