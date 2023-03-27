use serde::{
    Serialize,
    Deserialize
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub dispatcher: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub errorMessage: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub parameters: Parameters,
    pub workflow: Workflow
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub event: String,
    pub runId: String,
    pub runName: String,
    pub utcTime: String,
    pub metadata: Metadata,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct BaseMessage {
    pub event: String,
    pub runId: String,
    pub runName: String,
    pub utcTime: String
}