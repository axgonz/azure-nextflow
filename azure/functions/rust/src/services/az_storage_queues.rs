use az_app_identity::*;

use crate::app::variables::*;

use azure_storage:: {
    StorageCredentials
};
use azure_storage_queues:: {
    QueueServiceClient,
    QueueClient
};

use actix_web::web::Json;
use serde_json::Value;

#[derive(Clone)]
pub struct AppAzStorageQueue {
    pub client: QueueClient,
}

impl AppAzStorageQueue { 
    pub fn new(credential: Arc<DefaultAzureCredential>, variables: &AppVariables) -> Self {
        let storage_credentials = StorageCredentials::TokenCredential(credential);
        let queue_service = QueueServiceClient::new(&variables.nxfutil_az_st_name, storage_credentials);
        Self {
            client: queue_service.queue_client("nextflow")
        }
    }
    
    #[allow(dead_code)]
    pub async fn send_message_to_queue(
        credential: Arc<DefaultAzureCredential>, 
        variables: &AppVariables,
        req_payload: Json<Value>
    ) {
        let queue = AppAzStorageQueue::new(credential.clone(), &variables);
        match queue.client.put_message(req_payload.to_string()).await {
            Ok(_) => {
                println!("[handler][az-storage-queues] Sending message...Ok");
            },
            Err(error) => {
                println!("[handler][az-storage-queues] Sending message...Err");
                println!("{}", error)
            }        
        }
    }

    pub async fn peak_message_from_queue(
        credential: Arc<DefaultAzureCredential>, 
        variables: &AppVariables,
        count: u8
    ) -> Vec<Value> {       
        if count < 1 {
            return vec![];
        }
        let mut messages: Vec<Value> = vec![]; 
        let queue = AppAzStorageQueue::new(credential.clone(), &variables);
        match queue.client.peek_messages().number_of_messages(count).await {
            Ok(response) => {
                println!("[handler][az-storage-queues] Peak message...Ok");
                for message in response.messages {
                    let raw_msg: Value = serde_json::from_str(&message.message_text).unwrap();
                    messages.push(raw_msg);
                }
            },
            Err(error) => {
                println!("[handler][az-storage-queues] Peak message...Err");
                println!("{}", error);
            }        
        };
        return messages
    }
    
    pub async fn get_message_from_queue(
        credential: Arc<DefaultAzureCredential>, 
        variables: &AppVariables,
        count: u8
    ) -> Vec<Value> {       
        if count < 1 {
            return vec![];
        }
        let mut messages: Vec<Value> = vec![]; 
        let queue = AppAzStorageQueue::new(credential.clone(), &variables);
        match queue.client.get_messages().number_of_messages(count).await {
            Ok(response) => {
                println!("[handler][az-storage-queues] Get messages...Ok");
                for message in response.messages {
                    let raw_msg: Value = serde_json::from_str(&message.message_text).unwrap();
                    messages.push(raw_msg);
                    match queue.client.pop_receipt_client(message).delete().await {
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