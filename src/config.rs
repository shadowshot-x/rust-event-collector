use serde::Deserialize;
use std::fs;


#[derive(Debug, Deserialize)]
pub struct FlushFolder {
    pub mode : String,
    pub location: String,
    pub rotation_threshold: i32
}

#[derive(Debug, Deserialize)]
pub struct Config{
    pub counter_file : String,
    pub aggregate_file : String,
    pub tail_folder : String,
    pub audit_file: String,
    pub flush: FlushFolder
}

pub fn config_unmarshall(config_file_location: &String) -> Config {
    let data = fs::read_to_string(config_file_location).expect("Unable to read config file");

    let conf_var: Config = serde_json::from_str(&data).unwrap();

    conf_var
}