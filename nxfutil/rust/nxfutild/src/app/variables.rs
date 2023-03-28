pub use az_app_variables::*;

#[derive(AzAppVariablesNew, AzAppVariablesInit, Debug)]
pub struct AppVariables {
    pub nxfutil_az_st_name: String,
}
