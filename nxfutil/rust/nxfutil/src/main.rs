mod args;
mod app;
mod services;

use args::*;

use az_app_identity::*;

use app::{
    variables::*,
    secrets::*,
    server::*,
    config_parser::*,
};

use std::{
    time::Duration,
    thread,
};

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let app_identity = AppIdentity::new();

    let mut app_variables = AppVariables::new();
    AppVariables::init(&mut app_variables);

    let mut app_secrets = AppSecrets::new();
    AppSecrets::init(&mut app_secrets, &app_variables.nxfutil_az_kv_name, app_identity.clone()).await;

    // Prase --parameters_json as JSON string
    let nextflow_params: Vec<NextflowParam> = match args.parameters_json {
        Some(value) => {
            match serde_json::from_str(&value) {
                Ok(value) => {
                    value
                }
                Err(_) => {
                    let nextflow_param: NextflowParam = match serde_json::from_str(&value) {
                        Ok(value) => {
                            value
                        }
                        Err(error) => {
                            /* slow_panic! */ 
                            println!("Unable to parse --params as serialized JSON string");
                            println!("{:#?}", error);
                            //ToDo send_message to queue before exiting.
                            println!("[app] About to panic!");
                            if args.auto_delete {
                                AppServer::terminate(&app_variables).await;
                            }
                            panic!();
                            /* end slow_panic! */
                        }
                    };
                    vec![nextflow_param]
                }
            }
        }
        None => {
            vec![]
        }
    };  

    // Coerce types for passing to std::process::Command
    let mut nextflow_param_strings: Vec<String> = vec![];
    for param in nextflow_params {
        let name: String = format!("--{}", param.name);
        let value: String = match param.value.as_str() {
            Some(value) => format!("{}", value),
            None => format!("{}", param.value.to_string())
        };
        nextflow_param_strings.push(name);
        nextflow_param_strings.push(value);
    }

    // Push on nxfutil args for ease of logging 
    nextflow_param_strings.push(String::from("--nxfutil_config_uri"));
    nextflow_param_strings.push(format!("{}", args.config_uri));
    nextflow_param_strings.push(String::from("--nxfutil_pipeline_uri"));
    nextflow_param_strings.push(format!("{}", args.pipeline_uri));
    nextflow_param_strings.push(String::from("--nxfutil_parameters_uri"));
    nextflow_param_strings.push(format!("{}", args.parameters_uri));
    nextflow_param_strings.push(String::from("--nxfutil_auto_delete"));
    nextflow_param_strings.push(format!("{}", args.auto_delete));

    println!("[app] Downloading nextflow files (.config, .nf, .json)...");
    AppServer::web_download(&args.config_uri.to_string(), &"nextflow.config".to_string()).await;
    AppServer::web_download(&args.pipeline_uri.to_string(), &"pipeline.nf".to_string()).await;
    AppServer::web_download(&args.parameters_uri.to_string(),&"parameters.json".to_string()).await;
    
    println!("[app] Parsing nextflow config...");
    ConfigParser::parse_secrets("nextflow.config", &app_variables, app_identity.clone()).await;
    ConfigParser::parse_extended_params("nextflow.config", &app_variables, app_identity.clone()).await;

    println!("[app] Starting nxfutild service...");
    let mut nxfutild = AppServer::nxfutild();

    println!("[app] Showing nextflow config...");
    let mut nextflow_exit_code = AppServer::nextflow(vec![
        "config"
    ]);
    if nextflow_exit_code > 0 {
        /* slow_panic! */ 
        println!("Nextflow process did not run cleanly. ExitCode: {:#?}", nextflow_exit_code);
        //ToDo send_message to queue before exiting.
        println!("[app] About to panic!");
        if args.auto_delete {
            AppServer::terminate(&app_variables).await;
        }
        panic!();
        /* end slow_panic! */
    };

    // Wait for the subprocess to be ready
    let mut status = 0;
    let mut timeout = 25;
    let delay_seconds = 5;
    let delay_duration = Duration::from_secs(delay_seconds);
    while status != 200 {
        if timeout == 0 {
            /* slow_panic! */ 
            println!("Unable to start nxfutild service.");
            //ToDo send_message to queue before exiting.
            println!("[app] About to panic!");
            if args.auto_delete {
                AppServer::terminate(&app_variables).await;
            }
            panic!();
            /* end slow_panic! */
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
        app_variables.nxfutil_dispatcher.as_str()
    ];    
    nextflow_cmd.append(&mut nextflow_param_strings.iter().map(String::as_ref).collect());

    println!("[app] Handing over to nextflow...");
    nextflow_exit_code = AppServer::nextflow(nextflow_cmd);
    if nextflow_exit_code > 0 {
        /* slow_panic! */ 
        println!("Nextflow process did not run cleanly. ExitCode: {:#?}", nextflow_exit_code);
        //ToDo send_message to queue before exiting.
        println!("[app] About to panic!");
        if args.auto_delete {
            AppServer::terminate(&app_variables).await;
        }
        panic!();
        /* end slow_panic! */
    };

    if args.auto_delete {
        AppServer::terminate(&app_variables).await;
    }

    println!("[app] Stopping nxfutild service...");
    nxfutild.kill().expect("Unable to stop nxfutild service.");

    println!("[app] Done!");
}