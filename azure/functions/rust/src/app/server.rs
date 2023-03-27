#[derive(Clone)]
pub struct AppServer {
    variables: AppVariables,
    az_identity: AppAzIdentity,
    az_storage_queues: AppAzStorageQueues
}

impl AppServer {
    fn new(variables: AppVariables, az_identity: AppAzIdentity) -> Self {
        Self {
            az_storage_queues: AppAzStorageQueues::new(az_identity.credential.clone(), &variables),
            az_identity: az_identity,
            variables: variables,
        }
    }
    async fn init(server: &AppServer) {
        match server.az_storage_queues.queue_client.create().await {
            Ok(_) => {
                println!("[handler][az-storage-queues] Creating queue if not exists {:#?}...Ok", server.variables.q_name);
            },
            Err(error) => {
                println!("[handler][az-storage-queues] Creating queue if not exists {:#?}...Err", server.variables.q_name);
                panic!("{}", error)
            }
        }
    }
    #[allow(dead_code)]
    async fn send_message_to_queue(Json(req_payload): Json<Value>, server: &AppServer) {
        match server.az_storage_queues.queue_client.put_message(req_payload.to_string()).await {
            Ok(_) => {
                println!("[handler][az-storage-queues] Sending message...Ok");
            },
            Err(error) => {
                println!("[handler][az-storage-queues] Sending message...Err");
                println!("{}", error)
            }        
        }
    }
    async fn peak_message_from_queue(count: u8, server: &AppServer) -> Vec<Value> {       
        if count < 1 {
            return vec![];
        }
        
        let mut messages: Vec<Value> = vec![]; 
        match server.az_storage_queues.queue_client.peek_messages().number_of_messages(count).await {
            Ok(response) => {
                println!("[handler][az-storage-queues] Peak message...Ok");
                for message in response.messages {
                    let raw_msg: Value = serde_json::from_str(&message.message_text).unwrap();
                    messages.push(Self::clean_nextflow_message(&raw_msg));
                }
            },
            Err(error) => {
                println!("[handler][az-storage-queues] Peak message...Err");
                println!("{}", error);
            }        
        };

        return messages
    }
    async fn get_message_from_queue(count: u8, server: &AppServer) -> Vec<Value> {       
        if count < 1 {
            return vec![];
        }
        let mut messages: Vec<Value> = vec![]; 
        match server.az_storage_queues.queue_client.get_messages().number_of_messages(count).await {
            Ok(response) => {
                println!("[handler][az-storage-queues] Get messages...Ok");
                for message in response.messages {
                    let raw_msg: Value = serde_json::from_str(&message.message_text).unwrap();
                    messages.push(Self::clean_nextflow_message(&raw_msg));
                    match server.az_storage_queues.queue_client.pop_receipt_client(message).delete().await {
                        Ok(_) => {
                            println!("[handler][az-storage-queues] Delete message...Ok");
                        }
                        Err(error) => {
                            println!("[handler][az-storage-queues] Delete message...Err");
                            println!("{}", error);
                        }
                    }
                }
            },
            Err(error) => {
                println!("[handler][az-storage-queues] Get messages...Err");
                println!("{}", error);
            }        
        };
        return messages
    }    
    fn clean_nextflow_message(raw_msg: &Value) -> Value {
        let error_message: Option<String> = serde_json::from_value(raw_msg["metadata"]["workflow"]["errorMessage"].clone()).unwrap();
        let error_report: Option<String> = serde_json::from_value(raw_msg["metadata"]["workflow"]["errorReport"].clone()).unwrap();
        if error_message.is_some() || error_report.is_some() {
            let mut clean_msg = raw_msg.clone();
            clean_msg["event"]="error".to_string().into();
            return clean_msg
        }
        else {
            return raw_msg.clone()
        }
    }
}