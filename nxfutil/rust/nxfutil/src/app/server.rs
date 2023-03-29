use az_app_identity::*;

use crate::app::variables::*;
use crate::services_raw::az_mgmt_containerinstance::*;

use reqwest::{
    Response,
    Error as ReqwestError
};

use serde_json::Value;

use std::{
    error::Error,
};

use std::process::{
    Command,
    Child,
    Stdio,
};

#[derive(Clone)]
pub struct AppServer {}

impl AppServer {
    pub async fn web_status(uri: &String) -> u16 {
        match reqwest::get(uri).await {
            Ok(response) => {
                println!("[reqwest] (status) GET {:#?}...Ok", uri);
                return response.status().as_u16();
            }
            Err(_) => {
                println!("[reqwest] (status) GET {:#?}...Err", uri);
                return 404;
            }
        }
    }
    
    pub async fn web_get(uri: &String) -> Response {
        let response = match reqwest::get(uri).await {
            Ok(response) => {
                response
            }
            Err(error) => {
                println!("[reqwest] GET {:#?}...Err", uri);
                panic!("{}", error)
            }
        };
        if response.status() == 200 {
            println!("[reqwest] GET {:#?}...Ok", uri);
            return response
        }
        else {
            println!("[reqwest] GET {:#?}...Err", uri);
            panic!("{}", response.status())
        }
    }
    
    pub async fn web_download(uri: &String, destination: &String) {
        let response = Self::web_get(uri).await;
        let content = match response.text().await {
            Ok(content) => {
                content
            }
            Err(error) => {
                println!("[reqwest] SAVE {:#?}...Err", destination);
                panic!("{}", error)
            }
        };

        // N.B. This will not work well as file sizes get larger!
        match tokio::fs::write(&destination, &content.as_bytes()).await {
            Ok(_) => {
                println!("[reqwest] SAVE {:#?}...Ok", destination);
            }
            Err(error) => {
                println!("[reqwest] SAVE {:#?}...Err", destination);
                panic!("{}", error)
            }
        }
    }
    
    pub async fn web_post(uri: &String, json: &Value) -> Result<Response, ReqwestError> {
        let client = reqwest::Client::new();
        match client.post(uri).json(json).send().await {
            Ok(response) => {
                println!("[reqwest] POST {:#?}...Ok", uri);
                return Ok(response)
            }
            Err(error) => {
                println!("[reqwest] POST {:#?}...Err", uri);
                return Err(error)
            }
        };
    }    

    pub async fn terminate(variables: &AppVariables, credential: Arc<DefaultAzureCredential>) {
        let _deployment = AppAzMgmtContainerInstance::delete_nxfutil_ci(
            credential, 
            &variables, 
            &variables.nxfutil_dispatcher,
            true
        ).await;
    }

    pub fn pre_panic(msg: &str, error: &dyn Error) {
        println!("{}", msg);
        println!("{:#?}", error);
        println!("[app] About to panic!");
    }

    pub fn nxfutild() -> Child {
        match Command::new("./nxfutild").spawn() {
            Ok(process) => process,
            Err(error) => {
                Self::pre_panic("[app] Unable to start nxfutild service.", &error);
                panic!();
            }
        }
    }
    
    pub fn nextflow(args: Vec<&str>) -> (i32, String, String) {
        let mut omit_log = false;
        for arg in &args {
            if arg.to_lowercase() == "secrets" {
                omit_log = true;
            }
        }
        let nextflow = match Command::new("/.nextflow/nextflow")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
                Ok(process) => {
                    if omit_log {
                        println!("[nextflow] Process started with args: (redacted)");
                    }
                    else {
                        println!("[nextflow] Process started with args: {:#?}", args);
                    }
                    process
                }
                Err(error) => {
                    Self::pre_panic("[nextflow] Failed to start process", &error);
                    panic!();
                }
            };
            
        let output = match nextflow.wait_with_output() {
            Ok(output) => {
                println!("[nextflow] Process exited with status code: {:#?}", output.status.code().unwrap());
                output
            }
            Err(error) => {
                Self::pre_panic("[nextflow] Process crashed", &error);
                panic!("{}", error);
            }
        };
        
        let stdout = String::from_utf8(output.stdout.clone()).unwrap();
        let stderr = String::from_utf8(output.stderr.clone()).unwrap();
        println!("{}\n{}", stdout, stderr);
        
        return (output.status.code().unwrap(), stdout, stderr)
    }
}