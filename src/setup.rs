use clap::{App, Arg};
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Serialize)]
struct Config{
    setup_config: SetupConfig
}

#[derive(Deserialize, Serialize)]
struct SetupConfig{
    setup_mode: SetupKind,
    add_hidden_flag: bool,
    included_dirs: Vec<String>
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
enum SetupKind {
    Default,
    Minimal,
    Standard,
    Maximal,
}

//run with superuser permissions
fn main() {
    let mut setup_mode: SetupKind = SetupKind::Minimal;
    let mut included_dirs: Vec<String> = Vec::new();
    let mut add_hidden_flag: bool = false;

    let app = App::new("file_search setup")
        .version("0.1")
        // .author("Ahmet Pehlivanoglu ahmet.pehlivanoglu@ozu.edu.tr")
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
                .multiple_values(true) // Allows multiple values
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

    if let Some(values) = app.values_of("include") {
        included_dirs = values.map(|s| s.to_string()).collect::<Vec<String>>();
        println!("Default configuration selected with: ");
    }
    println!("{} directories", included_dirs.join(", "));

    if app.is_present("setup_mode") {
        setup_mode = match app.value_of("setup_mode").unwrap().to_lowercase().as_str() {
            "default" => SetupKind::Default,
            "minimal" => SetupKind::Minimal,
            "standard" => SetupKind::Standard,
            "maximal" => SetupKind::Maximal,
            _ => panic!("Setup mode type is invalid"),
        }
    };
    println!("Setup mode: {}", app.value_of("setup_mode").unwrap());



    let config = Config {
        setup_config: SetupConfig {
            setup_mode: setup_mode.clone(),
            add_hidden_flag,
            included_dirs: included_dirs.clone(),
        },
    };

    let toml_string = match toml::to_string(&config){
        Ok(t_string) => t_string,
        Err(err_msg) => panic!("Could not convert config params to string due to: {}", err_msg)
    };

    let _is_written = match fs::write("/etc/file_search/config.toml", toml_string){
        Ok(_) => println!("Config written to /etc/file_search/config.toml"),
        Err(err_msg) => panic!("Could not write to config file due to: {}", err_msg)
    };

    create_lib_dir();
    create_dbs();

    populate_db(setup_mode, included_dirs, add_hidden_flag);


    create_index_on_tables();
    println!("Database setup is complete!");
}

fn create_index_on_tables() {
    for c in 'a'..='z' {
        let path: String = format!("/var/lib/file_search/{}.db", c);
        let conn = Connection::open(path).unwrap();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_filename ON files(filename);",
            params![],
        )
        .unwrap();
    }
    for c in 'A'..='Z' {
        let path: String = format!("/var/lib/file_search/{}.db", c);
        let conn = Connection::open(path).unwrap();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_filename ON files(filename);",
            params![],
        )
        .unwrap();
    }
    let path: String = String::from("/var/lib/file_search/_.db");
    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_filename ON files(filename);",
        params![],
    )
    .unwrap();
}

fn create_dbs_util(start: char, end: char) {
    for c in start..=end {
        let path: String = format!("/var/lib/file_search/{}.db", c);
        if !Path::new(&path).exists() {
            let file_result = fs::File::create(&path);
            match file_result {
                Ok(_) => println!("Database {} successfully created!", &path),
                Err(err_msg) => panic!("Database could not be created due to: {}", err_msg),
            }
            let connection = Connection::open(&path).unwrap();
            connection
                .execute(
                    "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            filename TEXT NOT NULL
        )",
                    [],
                )
                .unwrap();
        }
    }
}

fn create_dbs() {
    create_dbs_util('a', 'z');
    create_dbs_util('A', 'Z');
    create_dbs_util('0', '9');
    create_dbs_util('_', '_');
}

fn populate_db(setup_mode: SetupKind, mut include_dirs: Vec<String>, add_hidden_flag: bool) {
    let mut count: i128 = 0;
    // println!("{}",include_dirs.get(0).unwrap());
    //.filter(move |e| include_dirs.iter().any(|&inc| e.path().starts_with(inc)))

    let minimal_dirs: Vec<String> = vec!["/home", "/bin", "/usr", "/root"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let standard_dirs: Vec<String> = vec!["/home", "/bin", "/usr", "/var", "/cdrom", "/etc", "/media", "/sbin", "/srv", "/root"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let excluded_maximal_dirs: Vec<String> = vec!["/proc", "/run", "/lost+found", "/tmp", "/dev"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    include_dirs = match setup_mode {
        SetupKind::Minimal => minimal_dirs.clone(),
        SetupKind::Standard => standard_dirs.clone(),
        SetupKind::Maximal => excluded_maximal_dirs.clone(),
        SetupKind::Default => include_dirs,
    };

    let directories: Box<dyn Iterator<Item = DirEntry>> = if setup_mode == SetupKind::Maximal {
        Box::new(
            WalkDir::new("/")
                .into_iter()
                .filter_map(Result::ok)
                .filter(move |e| !include_dirs.iter().any(|inc| e.path().starts_with(inc))),
        )
    } else {
        Box::new(
            WalkDir::new("/")
                .into_iter()
                .filter_map(Result::ok)
                .filter(move |e| include_dirs.iter().any(|inc| e.path().starts_with(inc))),
        )
    };

    for entry in directories {
        let path = entry.path();
        let condition: bool = match add_hidden_flag {
            false => {
                path.is_file()
                    && !path.components().any(|component| {
                        component
                            .as_os_str()
                            .to_str()
                            .unwrap_or("")
                            .starts_with(".")
                    })
            }
            true => path.is_file(),
        };
        if condition {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                // println!(
                //     "Currently in: {}",
                //     path.to_str().unwrap_or("[Invalid UTF-8]")
                // );
                let db_path = if let Some(first_char) =
                    filename.chars().next().filter(|c| c.is_alphanumeric())
                {
                    format!("/var/lib/file_search/{}.db", first_char)
                } else {
                    "/var/lib/file_search/_.db".to_string()
                };
                println!("{} --- {}", &db_path, filename);
                count += 1;

                let connection = Connection::open(&db_path).unwrap();
                let stmt = connection.prepare("INSERT INTO files (path, filename) VALUES (?, ?)");
                match stmt {
                    Ok(mut stmt) => {
                        let res = stmt.execute(params![path.to_str(), filename]);
                        // Handle the result of the SQL operation
                        if let Err(err) = res {
                            println!("Error inserting into {}: {}", db_path, err);
                        }
                    }
                    Err(e) => println!("Failed to prepare statement: {}", e),
                }
            }
        }
    }

    println!("{} files inserted!", count);
}

fn create_lib_dir() {
    let dir_path = "/var/lib/file_search";
    if !Path::new(dir_path).exists() {
        match fs::create_dir(dir_path) {
            Ok(_) => println!("Directory {} successfully created!", dir_path),
            Err(err_msg) => panic!("Directory could not be created due to: {}", err_msg),
        }
    } else {
        println!("Directory {} already exists!", dir_path);
    }
}