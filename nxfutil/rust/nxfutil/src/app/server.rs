use reqwest::{
    Response
};

#[derive(Clone)]
pub struct AppServer {
    pub variables: AppVariables,
    pub secrets: AppSecrets,
    pub az_identity: AppAzIdentity,
}

impl AppServer {
    fn new(variables: AppVariables, secrets: AppSecrets, az_identity: AppAzIdentity) -> Self {
        Self {
            az_identity: az_identity,
            variables: variables,
            secrets: secrets            
        }
    }
    async fn init(server: &AppServer) {
    }
    async fn web_status(uri: &String) -> u16 {
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
    async fn web_get(uri: &String) -> Response {
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
    async fn web_download(uri: &String, destination: &String) {
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
    async fn web_delete(uri: &String) -> Response {
        let client = reqwest::Client::new();
        let response = match client.delete(uri).send().await {
            Ok(response) => {
                response
            }
            Err(error) => {
                println!("[reqwest] DELETE {:#?}...Err", uri);
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
    fn nextflow(args: Vec<&str>) -> i32 {
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
                    println!("[nextflow] Failed to start");
                    panic!("{}", error)
                }
            };
        let exit_status: ExitStatus = match nextflow.wait() {
            Ok(status) => {
                println!("[nextflow] Process exited with status code: {:#?}", status.code().unwrap());
                status
            }
            Err(error) => {
                println!("[nextflow] Process crashed");
                panic!("{}", error)
            }
        };

        return exit_status.code().unwrap()
    }
}