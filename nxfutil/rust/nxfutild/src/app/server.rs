use crate::services::{
    az_storage_queues::*,
};

use actix_web::web::Json;
use serde_json::Value;

#[derive(Clone)]
pub struct AppServer {}

impl AppServer {
    pub async fn send_status_message(req_payload: Json<Value>, queue: &AppAzStorageQueue) {
        if !(req_payload["event"] == "started") && !(req_payload["event"] == "completed") {
            println!("[nxfutild][az-storage-queues] Ignoring message {:#?}", req_payload["event"]);
            return
        }
        match queue.client.put_message(req_payload.to_string()).await {
            Ok(_) => {
                println!("[nxfutild][az-storage-queues] Sending message {:#?}...Ok", req_payload["event"]);
            },
            Err(error) => {
                println!("[nxfutild][az-storage-queues] Sending message {:#?}...Err", req_payload["event"]);
                println!("[nxfutild]{}", error)
            }        
        }
    }
}