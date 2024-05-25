use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use dirs::home_dir;
use rusqlite::{params, Connection};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use std::env;
use std::path::PathBuf;
use users::{get_user_by_name, User};
use dirs;
use users::os::unix::UserExt;

#[derive(PartialEq)]
pub enum SetupKind {
    Default,
    Minimal,
    Standard,
    Maximal,
}

pub static MINIMAL_DIRS: &'static [&'static str] = &["/home", "/bin", "/usr", "/root"];
pub static EXCLUDED_MAXIMAL_DIRS: &'static [&'static str] = &["/proc", "/run", "/lost+found", "/tmp", "/dev"];
pub static STANDARD_DIRS: &'static [&'static str] = &[
    "/home", "/bin", "/usr", "/var", "/cdrom", "/etc", "/media", "/sbin", "/srv", "/root",
];


pub fn create_index_on_tables() {
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

pub fn create_dbs_util(start: char, end: char) {
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

pub fn create_dbs() {
    create_dbs_util('a', 'z');
    create_dbs_util('A', 'Z');
    create_dbs_util('0', '9');
    create_dbs_util('_', '_');
}

pub fn populate_db(setup_mode: SetupKind, mut include_dirs: Vec<&str>, add_hidden_flag: bool) {
    let mut count: i128 = 0;

    include_dirs = match setup_mode {
        SetupKind::Minimal => MINIMAL_DIRS.to_vec(),
        SetupKind::Standard => STANDARD_DIRS.to_vec(),
        SetupKind::Maximal => EXCLUDED_MAXIMAL_DIRS.to_vec(),
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

pub fn create_lib_dir() {
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
pub fn delete_lib_dir() {
    let dir_path = "/var/lib/file_search";
    if Path::new(dir_path).exists() {
        match fs::remove_dir_all(dir_path) {
            Ok(_) => println!("Directory {} successfully deleted!", dir_path),
            Err(err_msg) => panic!("Directory could not be deleted due to: {}", err_msg),
        }
    } else {
        println!("Directory {} does not exist!", dir_path);
    }
}
pub fn read_config_file() -> (Vec<String>, SetupKind, bool) {

    let mut config_path = get_home_dir();
    config_path.push(".config/file_search/config.toml");

    let config_file = match File::open(&config_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open config file due to: {}", err),
    };

    let mut reader = BufReader::new(config_file);
    let mut include_dirs: Vec<String> = Vec::new();

    let mut setup_mode_str: String = String::new();
    reader.read_line(&mut setup_mode_str).unwrap();
    setup_mode_str = setup_mode_str.trim().to_string();


    let mut add_hidden_str: String = String::new();
    reader.read_line(&mut add_hidden_str).unwrap();
    add_hidden_str = add_hidden_str.trim().to_string();

    let add_hidden_flag = match add_hidden_str.as_str() {
        "true" | "yes" | "1" => true,
        "false" | "no" | "0" => false,
        _ => true
    };


    let setup_mode = match setup_mode_str.as_str() {
        "standard" => SetupKind::Standard,
        "minimal" => SetupKind::Minimal,
        "maximal" => SetupKind::Maximal,
        "default" => SetupKind::Default,
        _ => panic!("Wrong config option")
    };

    for line in reader.lines() {
        include_dirs.push(line.unwrap().clone());
    }

    (include_dirs, setup_mode, add_hidden_flag)
}
pub fn create_config_file(setup_mode: &SetupKind, default_params: Vec<String>, add_hidden_flag: bool) {
    let config_str = match setup_mode {
        SetupKind::Standard => "standard",
        SetupKind::Minimal => "minimal",
        SetupKind::Maximal => "maximal",
        SetupKind::Default => "default"
    };

    let mut config_path = get_home_dir();

    config_path.push(".config/file_search");

    if let Err(err) = fs::create_dir_all(&config_path) {
        panic!("Could not create directories due to: {}", err);
    }

    config_path.push("config.toml");

    let mut config_file = match File::create(&config_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open to write config file due to: {}", err),
    };

    if let Err(err) = config_file.write_all(config_str.as_bytes()) {
        panic!("Could not write to config file due to: {}", err);
    }

    match add_hidden_flag {
        true => config_file.write_all("\ntrue".as_bytes()).expect(""),
        false => config_file.write_all("\nfalse".as_bytes()).expect("")
    }
    for dir in default_params{
        config_file.write_all(format!("\n{}",dir).as_bytes()).expect("");
    }
}
fn get_home_dir() -> PathBuf {
    if let Ok(sudo_user) = env::var("SUDO_USER") {
        if let Some(user_info) = get_user_by_name(&sudo_user) {
            return PathBuf::from(user_info.home_dir());
        }
    }
    dirs::home_dir().expect("Could not find home directory")
}