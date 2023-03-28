use az_app_identity::*;

use crate::app::{
    variables::*,
    secrets::*,
    server::*,
};

use regex::{
    Regex
};

use std::{
    io::Write,
    fs,
};

#[derive(Clone)]
pub struct ConfigParser {}

impl ConfigParser {
    fn find_in_string(string: &str, pattern: &str) -> Vec<String> {
        return Regex::new(pattern).unwrap().find_iter(string).map(|x| x.as_str().to_string()).collect();
    }
    
    fn find_in_file(file_name: &str, pattern: &str) -> Vec<String> {
        let string = fs::read_to_string(file_name).unwrap();
        return Self::find_in_string(&string, pattern);
    }
    
    fn find_unique_val_from_keyval_pattern(file_name: &str, pattern: &str) -> Vec<String> {
        let mut unique_names: Vec<String> = vec![];
        let matches: Vec<String> = Self::find_in_file(file_name, pattern);
        
        for item in matches {
            let split_item: Vec<&str> = item.split(".").collect();
            let name = split_item[1].to_string();

            if !unique_names.contains(&name) {
                unique_names.push(name);
            }
        };
        
        return unique_names
    }
    
    fn find_secrets(file_name: &str) -> Vec<String> {
        return Self::find_unique_val_from_keyval_pattern(file_name, r"secrets.[a-z,A-Z,_]*")
    }
    
    fn find_extended_params(file_name: &str) -> Vec<String> {
        return Self::find_unique_val_from_keyval_pattern(file_name, r"exParams.[a-z,A-Z,_]*")
    }
    
    pub async fn parse_secrets(file_name: &str, variables: &AppVariables, credential: Arc<DefaultAzureCredential>) {
        let config_secret_names = Self::find_secrets(file_name);
        
        for name in config_secret_names {
            let value = AppSecrets::get_from_key_vault(&name.replace("_","-"), &variables.nxfutil_az_kv_name, credential.clone()).await;
            AppServer::nextflow(vec![
                "secrets",
                "set",
                name.as_str(),
                value.as_str()
            ]);
        }
    }
    
    pub async fn parse_extended_params(file_name: &str, variables: &AppVariables, credential: Arc<DefaultAzureCredential>) {
        let config_param_names = Self::find_extended_params(file_name);
        let mut string = fs::read_to_string(file_name).unwrap();

        for name in config_param_names {
            let value = AppSecrets::get_from_key_vault(&name.replace("_","-"), &variables.nxfutil_az_kv_name, credential.clone()).await;
            string = string.replace(&format!("exParams.{}", name), &value);
            println!("[config-parser] exParam {:#?} replaced with {:#?}", name, value)
        }

        fs::File::create(file_name).unwrap()
            .write_all(string.as_bytes()).unwrap();
    }
}   