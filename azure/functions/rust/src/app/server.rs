#[derive(Clone)]
pub struct AppServer {
    variables: AppVariables,
    secrets: AppSecrets,
    az_identity: AppAzIdentity,
    az_storage_queues: AppAzStorageQueues
}

impl AppServer {
    fn new(variables: AppVariables, secrets: AppSecrets, az_identity: AppAzIdentity) -> Self {
        Self {
            az_storage_queues: AppAzStorageQueues::new(az_identity.credential.clone(), &variables, &secrets),
            az_identity: az_identity,
            variables: variables,
            secrets: secrets
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
                    messages.push(serde_json::from_str(&message.message_text).unwrap());
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
                    messages.push(serde_json::from_str(&message.message_text).unwrap());
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
}