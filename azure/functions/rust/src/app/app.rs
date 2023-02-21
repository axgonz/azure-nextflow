#[derive(Debug, Deserialize, Serialize)]
pub struct DispatchRequestPayload {
    pub config_uri: String,
    pub pipeline_uri: String,
    pub parameters_uri: String
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
pub struct Parameters {
    pub dispatcher: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub errorMessage: Option<String>,
    pub errorReport: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub parameters: Parameters,
    pub workflow: Workflow
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub event: String,
    pub runId: String,
    pub runName: String,
    pub utcTime: String,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseMessage {
    pub event: String,
    pub runId: String,
    pub runName: String,
    pub utcTime: String
}

pub struct App {}

impl App {
    pub async fn generate_status_update(server: &AppServer) -> Vec<Message>  {
        let raw_msgs: Vec<Value> = AppServer::peak_message_from_queue(32, server).await;
        let mut msgs: Vec<Message> = vec![];
        let mut msg_ids: Vec<String> = vec![];
        for raw_msg in raw_msgs.into_iter().rev() {
            if !(raw_msg["event"] == "started") && !(raw_msg["event"] == "completed") {
                // Error events are skinny and do not contain error details.
                // All other events are too verbose.
                continue
            }
            if !msg_ids.contains(&raw_msg["runId"].to_string()) {
                msg_ids.push(raw_msg["runId"].to_string());

                let mut msg: Message = serde_json::from_value(raw_msg.clone()).unwrap();
                if msg.metadata.workflow.errorMessage.is_some() || 
                    msg.metadata.workflow.errorReport.is_some() {
                    msg.event = "error".to_string();
                }  
                
                msgs.push(msg);
            }
        }
        return msgs
    }
    pub fn generate_nxfutil_cmd(req_payload: DispatchRequestPayload, url_params: HashMap<String, String>) -> String {
        /* nxfutil options for specifying 'config', 'pipeline' and 'parameters' files can
            be provided as either a json payload or as url arguments. Payload will take 
            precedence and url arguments will be depreciated.
        */
        println!("[handler] Checking url arguments for `config_uri`");
        let mut config_uri: String = match url_params.get("config_uri") {
            Some(key_value) => {
                println!("[handler] Found 'config_uri' url param {:#?}", key_value);
                key_value.to_string()
            },
            None => {
                "".to_string()
            }
        };
        println!("[handler] Checking RequestPayload for `config_uri`");
        if !req_payload.config_uri.is_empty() {
            println!("[handler] Found 'config_uri' in RequestPayload {:#?}", req_payload.config_uri);
            config_uri = req_payload.config_uri;
        }
        
        println!("[handler] Checking url arguments for `pipeline_uri`");
        let mut pipeline_uri: String = match url_params.get("pipeline_uri") {
            Some(key_value) => {
                println!("[handler] Found 'pipeline_uri' url param {:#?}", key_value);
                key_value.to_string()
            },
            None => {
                "".to_string()
            }
        };
        println!("[handler] Checking RequestPayload for `pipeline_uri`");
        if !req_payload.pipeline_uri.is_empty() {
            println!("[handler] Found 'pipeline_uri' in RequestPayload {:#?}", req_payload.pipeline_uri);
            pipeline_uri = req_payload.pipeline_uri;
        }

        println!("[handler] Checking url arguments for `parameters_uri`");
        let mut parameters_uri: String = match url_params.get("parameters_uri") {
            Some(key_value) => {
                println!("[handler] Found 'parameters_uri' url param {}", key_value);
                key_value.to_string()
            },
            None => {
                "".to_string()
            }
        };
        println!("[handler] Checking RequestPayload for `parameters_uri`");
        if !req_payload.parameters_uri.is_empty() {
            println!("[handler] Found 'parameters_uri' in RequestPayload {:#?}", req_payload.parameters_uri);
            parameters_uri = req_payload.parameters_uri;
        }

        let mut nxfutil_cmd: String = "nxfutil".to_string();
        if !config_uri.is_empty() {
            nxfutil_cmd = format!("{} -c {}", nxfutil_cmd, config_uri);
        }
        if !pipeline_uri.is_empty() {
            nxfutil_cmd = format!("{} -p {}", nxfutil_cmd, pipeline_uri);
        }
        if !parameters_uri.is_empty() {
            nxfutil_cmd = format!("{} -a {}", nxfutil_cmd, parameters_uri);
        }
        println!("[handler] Generated nextflow cmd is {:#?}", &nxfutil_cmd);
        
        return nxfutil_cmd
    }
}