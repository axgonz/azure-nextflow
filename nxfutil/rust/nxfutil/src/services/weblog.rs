use crate::models::nextflow_message::*;
use crate::app::server::*;

use std::time::SystemTime;

use chrono::{
    DateTime,
    Utc,
};

#[derive(Clone)]
pub struct Weblog {}

impl Weblog {
    pub async fn send_started(dispatcher: String) {
        println!("[weblog] send_started {:#?}", dispatcher);
        
        let datetime: DateTime<Utc> = SystemTime::now().into();

        let message = Message {
            event: "started".to_string(),
            runId: dispatcher.clone(),
            runName: dispatcher.clone(),
            utcTime: datetime.to_rfc3339(),
            metadata: Metadata {
                parameters: Parameters {
                    dispatcher: dispatcher
                },
                workflow: Workflow {
                    errorMessage: None
                }
            }
        };

        if AppServer::web_post(
            &"http://localhost:3000/api/nxfutild".to_string(), 
            &serde_json::to_value(&message).unwrap()
        ).await.is_err() {
            println!("[weblog] Unable to post.");
            println!("[weblog] About to panic!");
            panic!();
        };
    }

    pub async fn send_completed(dispatcher: String) {
        println!("[weblog] send_completed {:#?}", dispatcher);    

        let datetime: DateTime<Utc> = SystemTime::now().into();

        let message = Message {
            event: "completed".to_string(),
            runId: dispatcher.clone(),
            runName: dispatcher.clone(),
            utcTime: datetime.to_rfc3339(),
            metadata: Metadata {
                parameters: Parameters {
                    dispatcher: dispatcher
                },
                workflow: Workflow {
                    errorMessage: None
                }
            }
        }; 

        if AppServer::web_post(
            &"http://localhost:3000/api/nxfutild".to_string(), 
            &serde_json::to_value(&message).unwrap()
        ).await.is_err() {
            println!("[weblog] Unable to post.");
            println!("[weblog] About to panic!");
            panic!();
        };
    }

    pub async fn send_error(dispatcher: String, msg: String, stdout: String, stderr: String) {
        println!("{}", msg);
        println!("[weblog] send_error {:#?}", dispatcher);

        let datetime: DateTime<Utc> = SystemTime::now().into();

        let message = Message {
            event: "completed".to_string(),
            runId: dispatcher.clone(),
            runName: dispatcher.clone(),
            utcTime: datetime.to_rfc3339(),
            metadata: Metadata {
                parameters: Parameters {
                    dispatcher: dispatcher
                },
                workflow: Workflow {
                    errorMessage: Some(format!("{}\n{}\n{}", msg, stdout, stderr))
                }
            }
        };

        if AppServer::web_post(
            &"http://localhost:3000/api/nxfutild".to_string(), 
            &serde_json::to_value(&message).unwrap()
        ).await.is_err() {
            println!("[weblog] Unable to post.");
            println!("[weblog] About to panic!");
            panic!()
        };
    }
}