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
    pub name: String,
    pub client: QueueClient,
}

impl AppAzStorageQueue {
    pub fn new(name: &str, credential: Arc<DefaultAzureCredential>, variables: &AppVariables) -> Self {
        let storage_credentials = StorageCredentials::TokenCredential(credential);
        let queue_service = QueueServiceClient::new(&variables.nxfutil_az_st_name, storage_credentials);
        Self {
            name: name.to_string(),
            client: queue_service.queue_client(name)
        }
    }

    #[allow(dead_code)]
    pub async fn init(&self) {
        match self.client.create().await {
            Ok(_) => {
                println!("[az-storage-queues] Creating queue if not exists {:#?}...Ok", self.name);
            },
            Err(error) => {
                println!("[az-storage-queues] Creating queue if not exists {:#?}...Err", self.name);
                panic!("{}", error)
            }
        }
    }

    #[allow(dead_code)]
    pub async fn send_message_to_queue(
        &self,
        message: Json<Value>
    ) {
        match self.client.put_message(message.to_string()).await {
            Ok(_) => {
                println!("[az-storage-queues] Sending message...Ok");
            },
            Err(error) => {
                println!("[az-storage-queues] Sending message...Err");
                println!("{}", error)
            }
        }
    }

    #[allow(dead_code)]
    pub async fn peak_message_from_queue(
        &self,
        count: u8
    ) -> Vec<Value> {
        if count < 1 {
            return vec![];
        }
        let mut messages: Vec<Value> = vec![];
        match self.client.peek_messages().number_of_messages(count).await {
            Ok(response) => {
                println!("[az-storage-queues] Peak message...Ok");
                for message in response.messages {
                    let raw_msg: Value = serde_json::from_str(&message.message_text).unwrap();
                    messages.push(raw_msg);
                }
            },
            Err(error) => {
                println!("[az-storage-queues] Peak message...Err");
                println!("{}", error);
            }
        };
        return messages
    }

    #[allow(dead_code)]
    pub async fn get_message_from_queue(
        &self,
        count: u8
    ) -> Vec<Value> {
        if count < 1 {
            return vec![];
        }
        let mut messages: Vec<Value> = vec![];
        match self.client.get_messages().number_of_messages(count).await {
            Ok(response) => {
                println!("[az-storage-queues] Get messages...Ok");
                for message in response.messages {
                    let raw_msg: Value = serde_json::from_str(&message.message_text).unwrap();
                    messages.push(raw_msg);
                    match self.client.pop_receipt_client(message).delete().await {
                        Ok(_) => {
                            println!("[az-storage-queues] Delete message...Ok");
                        }
                        Err(error) => {
                            println!("[az-storage-queues] Delete message...Err");
                            println!("{}", error);
                        }
                    }
                }
            },
            Err(error) => {
                println!("[az-storage-queues] Get messages...Err");
                println!("{}", error);
            }
        };
        return messages
    }
}