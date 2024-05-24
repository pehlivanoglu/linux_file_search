use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use dirs::home_dir;

#[derive(PartialEq)]
pub enum SetupKind {
    Default,
    Minimal,
    Standard,
    Maximal,
}

fn main() {

    let mut config_path = home_dir().expect("Could not get home directory");
    config_path.push(".config/file_search");

    if let Err(err) = fs::create_dir_all(&config_path) {
        panic!("Could not create directories due to: {}", err);
    }

    config_path.push("config.toml");

    let mut config_file = match File::create(&config_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open to write config file due to: {}", err),
    };
    let default_params: Option<Vec<String>> = Some(vec!["a".to_string(),"b".to_string()]);

    for dir in default_params.unwrap(){
        config_file.write_all(format!("{}\n",dir).as_bytes()).expect("");
    }

}