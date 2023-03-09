include!("app/server.rs");
include!("app/variables.rs");
include!("app/secrets.rs");
include!("app/config-parser.rs");
include!("services/az-identity.rs");
include!("services/az-security-keyvault.rs");

mod args;
use args::*;

use std::{
    io::Write,
    env,
    time::Duration,
    thread,
    process::Command,
    process::ExitStatus,
    fs
};

use serde_json::{
    Value,
};

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let nextflow_params: Vec<NextflowParam> = match args.parameters_json {
        Some(value) => {
            match serde_json::from_str(&value) {
                Ok(value) => {
                    value
                }
                Err(_) => {
                    let nf_param: NextflowParam = match serde_json::from_str(&value) {
                        Ok(value) => {
                            value
                        }
                        Err(error) => {
                            println!("Unable to parse --params as serialized JSON string.");
                            panic!("{}", error);
                        }
                    };
                    vec![nf_param]
                }
            }
        }
        None => {
            vec![]
        }
    };  

    let mut nextflow_param_strings: Vec<String> = vec![];
    for param in nextflow_params {
        nextflow_param_strings.push(format!("--{}", param.name));
        nextflow_param_strings.push(format!("{}", param.value));
    }

    nextflow_param_strings.push(String::from("--nxfutilConfigUri"));
    nextflow_param_strings.push(format!("{}", args.config_uri));

    nextflow_param_strings.push(String::from("--nxfutilPipelineUri"));
    nextflow_param_strings.push(format!("{}", args.pipeline_uri));

    nextflow_param_strings.push(String::from("--nxfutilParametersUri"));
    nextflow_param_strings.push(format!("{}", args.parameters_uri));

    nextflow_param_strings.push(format!("--nxfutilAutoDelete"));
    nextflow_param_strings.push(format!("{}", args.auto_delete));

    let az_identity = AppAzIdentity::new();

    let mut variables = AppVariables::new();
    AppVariables::init(&mut variables);

    let mut secrets = AppSecrets::new(az_identity.credential.clone(), &variables);
    AppSecrets::init(&mut secrets).await;

    let server: AppServer = AppServer::new(variables, secrets, az_identity);

    println!("[app] Downloading nextflow files (.config, .nf, .json)...");
    AppServer::web_download(&args.config_uri.to_string(), &"nextflow.config".to_string()).await;
    AppServer::web_download(&args.pipeline_uri.to_string(), &"pipeline.nf".to_string()).await;
    AppServer::web_download(&args.parameters_uri.to_string(),&"parameters.json".to_string()).await;
    
    println!("[app] Parsing nextflow config...");
    ConfigParser::parse_secrets("nextflow.config", &server).await;
    ConfigParser::parse_extended_params("nextflow.config", &server).await;

    println!("[app] Starting nxfutild service...");
    let mut nxfutild = Command::new("./nxfutild")
        .env("FUNCTIONS_FUNCTION_NAME", "nxfutild")
        .spawn()
        .expect("Unable to start nxfutild service.");

    println!("[app] Showing nextflow config...");
    let mut nextflow_exit_code = AppServer::nextflow(vec![
        "config"
    ]);
    if nextflow_exit_code > 0 {
        //ToDo send_message to queue before exiting.
        println!("[app] Stopping nxfutild service...");
        nxfutild.kill().expect("Unable to stop nxfutild service.");
        panic!("Nexflow process did not run cleanly");
    };

    // wait for the subprocess to be ready
    let mut status = 0;
    let mut timeout = 25;
    let delay_seconds = 5;
    let delay_duration = Duration::from_secs(delay_seconds);
    while status != 200 {
        if timeout == 0 {
            panic!("Unable to start nxfutild service.");
        };
        status = AppServer::web_status(&"http://localhost:3000/api/nxfutild".to_string()).await;
        thread::sleep(delay_duration);
        timeout -= 1;
    }
    println!("[app] Service nxfutild is responding with {:#?}", status);

    println!("[app] Injecting --params into nextflow command...");
    let mut nextflow_cmd = vec![
        "run",
        "pipeline.nf",
        "-params-file",
        "parameters.json",
        "-w",
        "az://batch/work",
        "-with-weblog",
        "http://localhost:3000/api/nxfutild",
        "--dispatcher",
        server.variables.ci_name.as_str()
    ];    
    nextflow_cmd.append(&mut nextflow_param_strings.iter().map(String::as_ref).collect());

    println!("[app] Handing over to nextflow...");
    nextflow_exit_code = AppServer::nextflow(nextflow_cmd);
    if nextflow_exit_code > 0 {
        println!("Nexflow process did not run cleanly");
    };

    println!("[app] Stopping nxfutild service...");
    nxfutild.kill().expect("Unable to stop nxfutild service.");

    if args.auto_delete {
        println!("[app] Auto delete attempt...");

        let uri: String = format!("https://{}.azurewebsites.net/api/nxfutil/terminate", server.variables.fn_name);
        let json: Value = serde_json::from_str(&format!("{{\"ci_name\": \"{}\"}}", server.variables.ci_name)).unwrap();

        let mut status = 0;
        let mut retry = 5;
        let delay_seconds = 2;
        let delay_duration = Duration::from_secs(delay_seconds);
       
        while status != 200 {
            if retry == 0 {
                panic!("Unable to auto delete.");
            };
            match AppServer::web_post(&uri, &json).await {
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
            thread::sleep(delay_duration);
            retry -= 1;
        }
    }
}