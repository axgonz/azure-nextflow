use serde::{
    Serialize,
    Deserialize,
};

use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NextflowParam {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DispatchRequestPayload {
    pub config_uri: String,
    pub pipeline_uri: String,
    pub parameters_uri: String,
    pub parameters_json: Option<Vec<NextflowParam>>,
    pub auto_delete: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DispatchResponsePayload {
    pub sub_id: String,
    pub rg_name: String,
    pub ci_name: String,
    pub ci_cmd: String,
    pub provisioning_state: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TerminateRequestPayload {
    pub ci_name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TerminateResponsePayload {
    pub sub_id: String,
    pub rg_name: String,
    pub ci_name: String,
    pub provisioning_state: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusRequestPayload {
    pub summary: bool,
    pub message_count: u8,
    pub dequeue: bool
}

#[derive(Debug, Deserialize)]
pub struct QueryParameters {
    pub whatif: Option<bool>
}