use az_app_identity::*;

use crate::app::variables::*;
use crate::services::az_storage_queues::*;
use crate::models::api::*;
use crate::models::nextflow_message::*;

use actix_web::web::Json;
use serde_json::Value;

#[derive(Clone)]
pub struct AppServer {}

impl AppServer {
    pub fn generate_nxfutil_cmd(req_payload: Json<DispatchRequestPayload>) -> String {
        println!("[handler] Checking RequestPayload for `config_uri`");
        let mut config_uri = "".to_string();
        if !req_payload.config_uri.is_empty() {
            println!("[handler] Found 'config_uri' in RequestPayload {:#?}", req_payload.config_uri);
            config_uri = req_payload.config_uri.clone();
        }
        
        println!("[handler] Checking RequestPayload for `pipeline_uri`");
        let mut pipeline_uri = "".to_string();
        if !req_payload.pipeline_uri.is_empty() {
            println!("[handler] Found 'pipeline_uri' in RequestPayload {:#?}", req_payload.pipeline_uri);
            pipeline_uri = req_payload.pipeline_uri.clone();
        }

        println!("[handler] Checking RequestPayload for `parameters_uri`");
        let mut parameters_uri = "".to_string();
        if !req_payload.parameters_uri.is_empty() {
            println!("[handler] Found 'parameters_uri' in RequestPayload {:#?}", req_payload.parameters_uri);
            parameters_uri = req_payload.parameters_uri.clone();
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
        if req_payload.auto_delete {
            nxfutil_cmd = format!("{} -d", nxfutil_cmd);
        }
        match req_payload.parameters_json.clone() {
            Some(value) => {
                nxfutil_cmd = format!("{} -j '{}'", nxfutil_cmd, serde_json::to_string(&value).unwrap());
            }
            None => {}
        };
        println!("[handler] Generated nextflow cmd is {}", &nxfutil_cmd);
        
        return nxfutil_cmd
    }

    fn clean_nextflow_message(mut raw_msg: Value) -> Value {
        let error_message: Option<String> = serde_json::from_value(raw_msg["metadata"]["workflow"]["errorMessage"].clone()).unwrap();
        let error_report: Option<String> = serde_json::from_value(raw_msg["metadata"]["workflow"]["errorReport"].clone()).unwrap();
        
        if error_message.is_none() && error_report.is_some() {
            raw_msg["metadata"]["workflow"]["errorMessage"] = raw_msg["metadata"]["workflow"]["errorReport"].clone();
        }
        
        if error_message.is_some() || error_report.is_some() {
            let mut clean_msg = raw_msg.clone();
            clean_msg["event"]="error".to_string().into();
            return clean_msg
        }
        else {
            return raw_msg.clone()
        }
    }

    pub async fn get_status_message(
        credential: Arc<DefaultAzureCredential>, 
        variables: &AppVariables,
        count: u8, 
        dequeue: bool
    ) -> Vec<Value>  {
        let queue = AppAzStorageQueue::new("nextflow", credential.clone(), &variables);
        if dequeue {
            queue.get_message_from_queue(
                count
            ).await
                .iter()
                .map(|msg| Self::clean_nextflow_message(msg.clone()))
                .collect()
        } else {
            queue.peak_message_from_queue(
                count
            ).await
                .iter()
                .map(|msg| Self::clean_nextflow_message(msg.clone()))
                .collect()
        }
    }  

    pub async fn get_status_summary(
        credential: Arc<DefaultAzureCredential>, 
        variables: &AppVariables,
        count: u8, 
        dequeue: bool
    ) -> Vec<Value>  {
        let raw_msgs: Vec<Value>;
        raw_msgs = Self::get_status_message(credential, variables, count, dequeue).await;
        let mut msgs: Vec<Value> = vec![];
        let mut msg_ids: Vec<String> = vec![];

        // Iterate in reverse order to create summary
        for mut raw_msg in raw_msgs.into_iter().rev() {
            if !(raw_msg["event"] == "started") && 
                !(raw_msg["event"] == "completed") && 
                !(raw_msg["event"] == "error") {
                // Error events are skinny and do not contain error details.
                // All other events are too verbose.
                continue
            }
            if !msg_ids.contains(&raw_msg["runId"].to_string()) {
                msg_ids.push(raw_msg["runId"].to_string());

                // Cast to strict type Message to drop unwanted properties
                let msg: Message = serde_json::from_value(raw_msg).unwrap();

                // Cast back to Value to satisfy return type
                let raw_msg: Value = serde_json::from_str(&serde_json::to_string(&msg).unwrap()).unwrap();
                
                msgs.push(raw_msg);
            }
        }

        // Undo reverse ordering needed above
        let mut msgs_in_order: Vec<Value> = vec![];
        for item in msgs.iter().rev() {
            msgs_in_order.push(item.clone());
        }
        return msgs_in_order
    }
}