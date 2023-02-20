include!("app/server.rs");
include!("app/variables.rs");
include!("app/secrets.rs");
include!("app/config-parser.rs");
include!("services/az-identity.rs");
include!("services/az-security-keyvault.rs");

use std::{
    io,
    io::Write,
    env,
    time::Duration,
    thread,
    process::Command,
    process::ExitStatus,
    fs
};

use clap::{
    Parser
};

#[derive(Parser)]
struct Cli {
    // Uri to nextflow config ('.config') file
    #[arg(short = 'c', long, 
        default_value_t = ("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/nextflow.config".to_string())
    )]
    config_uri: String,

    // Uri to nextflow pipeline ('.nf') file
    #[arg(short = 'p', long, 
        default_value_t = ("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/pipeline.nf".to_string())
    )]
    pipeline_uri: String,
    
    // Uri to nextflow parameters ('.json') file
    #[arg(short = 'a', long, 
        default_value_t = ("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/parameters.json".to_string())
    )]
    parameters_uri: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let az_identity = AppAzIdentity::new();

    let mut variables = AppVariables::new();
    AppVariables::init(&mut variables);

    let mut secrets = AppSecrets::new(az_identity.credential.clone(), &variables);
    AppSecrets::init(&mut secrets).await;

    let mut server: AppServer = AppServer::new(variables, secrets, az_identity);

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

    println!("[app] Handing over to nextflow...");
    nextflow_exit_code = AppServer::nextflow(vec![
        "run",
        "pipeline.nf",
        "-params-file",
        "parameters.json",
        "-w",
        "az://batch/work",
        "-with-weblog",
        "http://localhost:3000/api/nxfutild"
    ]);
    if nextflow_exit_code > 0 {
        //ToDo send_message to queue before exiting.
        println!("[app] Stopping nxfutild service...");
        nxfutild.kill().expect("Unable to stop nxfutild service.");
        panic!("Nexflow process did not run cleanly");
    };

    println!("[app] Stopping nxfutild service...");
    nxfutild.kill().expect("Unable to stop nxfutild service.");
}