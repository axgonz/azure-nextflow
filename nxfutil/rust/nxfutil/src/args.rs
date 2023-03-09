pub use serde::{
    Deserialize, 
    Serialize,
};

pub use serde_json::{
    Value
};

pub use clap::{
    Args,
    Parser
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextflowParam {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Uri to nextflow config ('.config') file
    #[arg(
        short = 'c', 
        long, 
        default_value_t = String::from("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/nextflow.config")
    )]
    pub config_uri: String,

    /// Uri to nextflow pipeline ('.nf') file
    #[arg(
        short = 'p', 
        long, 
        default_value_t = String::from("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/pipeline.nf")
    )]
    pub pipeline_uri: String,
    
    /// Uri to nextflow parameters ('.json') file
    #[arg(
        short = 'a', 
        long, 
        default_value_t = String::from("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/parameters.json")
    )]
    pub parameters_uri: String,

    /// Nextflow parameters as a serialized JSON string '[{"name": "foo", "value": "bar"}]'.
    #[arg(
        short = 'j',
        long, 
        // default_value_t = String::from("[]")
    )]
    pub parameters_json: Option<String>,

    /// Try to delete parent container instance once complete
    #[arg(
        short = 'd', 
        long, 
        default_value_t = false
    )]
    pub auto_delete: bool,
}