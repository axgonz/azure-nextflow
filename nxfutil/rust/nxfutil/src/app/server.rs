use crate::app::variables::*;

use reqwest::{
    Response,
    Error as ReqwestError
};

use serde_json::Value;

use std::{
    error::Error,
    time::Duration,
    thread,
};


use std::process::{
    Command,
    Child,
    ExitStatus,
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
            //ToDo send_message to queue before exiting. 
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

    pub async fn terminate(variables: &AppVariables) {
        println!("[app] Auto delete attempt...");

        let uri: String = format!("https://{}/api/nxfutil/terminate", variables.nxfutil_api_fqdn);
        let json: Value = serde_json::from_str(&format!("{{\"ci_name\": \"{}\"}}", variables.nxfutil_dispatcher)).unwrap();

        let mut status = 0;
        let mut retry = 3;
        let mut delay = 3;
        
        while status != 200 && retry > 0 {
            match Self::web_post(&uri, &json).await {
                Ok(response) => {
                    let body: Value = response.json().await.unwrap();
                    println!("{}", body);
                    status = 200
                }
                Err(error) => {
                    println!("{}", error);
                    status = 400
                }
            };
            thread::sleep(Duration::from_secs(delay));
            delay += delay;
            retry -= 1;
        }
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
                Self::pre_panic("[app] Failed to start nxfutild service.", &error);
                panic!();
            }
        }
    }
    
    pub fn nextflow(args: Vec<&str>) -> i32 {
        let mut omit_log = false;
        for arg in &args {
            if arg.to_lowercase() == "secrets" {
                omit_log = true;
            }
        }
        let mut nextflow = match Command::new("/.nextflow/nextflow")
            .args(&args)
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
        let exit_status: ExitStatus = match nextflow.wait() {
            Ok(status) => {
                println!("[nextflow] Process exited with status code: {:#?}", status.code().unwrap());
                status
            }
            Err(error) => {
                Self::pre_panic("[nextflow] Process crashed", &error);
                panic!("{}", error);
            }
        };

        return exit_status.code().unwrap()
    }
}