use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;


#[derive(Deserialize)]
pub struct Config {
    pub output_path: String,
    pub input_path: String,
    pub key_path: String,
    pub is_encrypt: bool
}

#[derive(Deserialize)]
pub struct Data {
    pub config: Config
}

pub fn import_config(config_path: &str) -> Data {
    let contents = match fs::read_to_string(config_path) {

        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", config_path);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", config_path);
            exit(1);
        }
    };
    data

}