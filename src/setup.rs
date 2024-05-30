use clap::{App, Arg};
use std::fs;

use file_search_lib::{Config, SetupConfig, SetupKind, create_index_on_tables, create_dbs_util, populate_db, create_lib_dir, delete_lib_dir};

fn main() {
    let mut setup_mode = SetupKind::Minimal;
    let mut included_dirs = Vec::new();
    let mut add_hidden_flag = false;

    let app = App::new("file_search setup")
        .version("0.1")
        .about("Setup for file_search module")
        .arg(
            Arg::new("setup_mode")
                .long("setup_mode")
                .takes_value(true)
                .default_value("minimal"),
        )
        .arg(
            Arg::new("include")
                .long("include")
                .value_name("folder")
                .help("Includes files specified")
                .multiple_values(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("add_hidden")
                .long("add_hidden")
                .help("add hidden folders/files")
                .takes_value(false),
        )
        .get_matches();

    if app.is_present("add_hidden") {
        add_hidden_flag = true;
        println!("Considering hidden folders/files as well");
    }

    if let Some(mode) = app.value_of("setup_mode") {
        setup_mode = match mode.to_lowercase().as_str() {
            "default" => SetupKind::Default,
            "minimal" => SetupKind::Minimal,
            "standard" => SetupKind::Standard,
            "maximal" => SetupKind::Maximal,
            _ => {
                eprintln!("Setup mode type is invalid");
                return;
            }
        }
    }

    println!("Setup mode: {}", app.value_of("setup_mode").unwrap());

    if let Some(values) = app.values_of("include") {
        included_dirs = values.map(|s| s.to_string()).collect();
        println!("Default configuration selected with: {}", included_dirs.join(", "));
    }

    if setup_mode == SetupKind::Default && included_dirs.is_empty() {
        panic_message("At least 1 directory must be selected for default setup mode!");
    }

    let config = Config {
        setup_config: SetupConfig {
            setup_mode: setup_mode.clone(),
            add_hidden_flag,
            included_dirs: included_dirs.clone(),
        },
    };

    if let Err(err_msg) = save_config_to_file(&config, "/etc/file_search/config.toml") {
        panic_message(&format!(
            "Could not write to config file due to: {}",
            err_msg
        ));
    }

    delete_lib_dir();
    create_lib_dir();
    create_dbs();
    populate_db(setup_mode, included_dirs, add_hidden_flag);
    create_index_on_tables();
    println!("Database setup is complete!");
}

fn save_config_to_file(config: &Config, file_path: &str) -> Result<(), String> {
    let toml_string = toml::to_string(config)
        .map_err(|err_msg| format!("Could not convert config params to string due to: {}", err_msg))?;
    fs::write(file_path, toml_string)
        .map_err(|err_msg| format!("Could not write to config file due to: {}", err_msg))
}

fn create_dbs() {
    create_dbs_util('a', 'z');
    create_dbs_util('A', 'Z');
    create_dbs_util('0', '9');
    create_dbs_util('_', '_');
}

fn panic_message(message: &str) {
    println!("/*************************************************************/");
    println!("{}", message);
    panic!("/*************************************************************/");
}
