use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use walkdir::{DirEntry,WalkDir};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub setup_config: SetupConfig,
}

#[derive(Deserialize, Serialize)]
pub struct SetupConfig {
    pub setup_mode: SetupKind,
    pub add_hidden_flag: bool,
    pub included_dirs: Vec<String>,
}

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub enum SetupKind {
    Custom,
    Minimal,
    Standard,
    Maximal,
}

pub fn create_index(db_path: &str) {
    let conn = Connection::open(db_path).unwrap();
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_filename ON files(filename);",
        params![],
    )
        .unwrap();
}

pub fn create_dbs_util(start: char, end: char) {
    for c in start..=end {
        let path = format!("/var/lib/file_search/{}.db", c);
        if !Path::new(&path).exists() {
            fs::File::create(&path).expect("Database could not be created");
            let connection = Connection::open(&path).unwrap();
            connection.execute(
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

pub fn populate_db(setup_mode: SetupKind, mut include_dirs: Vec<String>, add_hidden_flag: bool) {
    let minimal_dirs = vec!["/home", "/bin", "/usr", "/root"];
    let standard_dirs = vec![
        "/home", "/bin", "/usr", "/var", "/cdrom", "/etc", "/media", "/sbin", "/srv", "/root",
    ];
    let excluded_maximal_dirs = vec!["/proc", "/run", "/lost+found", "/tmp", "/dev"];

    include_dirs = match setup_mode {
        SetupKind::Minimal => minimal_dirs.into_iter().map(String::from).collect(),
        SetupKind::Standard => standard_dirs.into_iter().map(String::from).collect(),
        SetupKind::Maximal => excluded_maximal_dirs.into_iter().map(String::from).collect(),
        SetupKind::Custom => include_dirs,
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

    let count = directories
        .filter_map(|entry| {
            let path = entry.path();
            let condition = if add_hidden_flag {
                path.is_file()
            } else {
                path.is_file()
                    && !path
                    .components()
                    .any(|component| component.as_os_str().to_str().unwrap_or("").starts_with("."))
            };

            if condition {
                path.file_name().and_then(|n| n.to_str()).map(|filename| {
                    let db_path = if filename.chars().next().filter(|c| c.is_alphanumeric()).is_some()
                    {
                        format!("/var/lib/file_search/{}.db", filename.chars().next().unwrap())
                    } else {
                        "/var/lib/file_search/_.db".to_string()
                    };

                    let connection = Connection::open(&db_path).unwrap();
                    let stmt = connection.prepare("INSERT INTO files (path, filename) VALUES (?, ?)");

                    if let Ok(mut stmt) = stmt {
                        if let Err(err) = stmt.execute(params![path.to_str(), filename]) {
                            println!("Error inserting into {}: {}", db_path, err);
                        }
                    } else {
                        println!("Failed to prepare statement");
                    }
                })
            } else {
                None
            }
        })
        .count();

    println!("{} file(s) inserted!", count);
}

pub fn create_lib_dir() {
    let dir_path = "/var/lib/file_search";
    if !Path::new(dir_path).exists() {
        fs::create_dir(dir_path).expect("Directory could not be created");
    } else {
        println!("Directory {} already exists!", dir_path);
    }
}

pub fn delete_lib_dir() {
    let dir_path = "/var/lib/file_search";
    if Path::new(dir_path).exists() {
        fs::remove_dir_all(dir_path).expect("Directory could not be deleted");
    } else {
        println!("Directory {} does not exist!", dir_path);
    }
}
pub fn create_index_on_tables() {
    for c in 'a'..='z' {
        create_index(&format!("/var/lib/file_search/{}.db", c));
    }
    for c in 'A'..='Z' {
        create_index(&format!("/var/lib/file_search/{}.db", c));
    }
    for c in '0'..='9' {
        create_index(&format!("/var/lib/file_search/{}.db", c));
    }
    create_index("/var/lib/file_search/_.db");
}