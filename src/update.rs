use rusqlite::{params, Connection};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};
use serde::{Deserialize,Serialize};
use daemonize::Daemonize;

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

#[derive(PartialEq, Debug, Deserialize, Serialize)]
enum SetupKind {
    Default,
    Minimal,
    Standard,
    Maximal,
}

//run with superuser permissions
fn main() {
    //
    // let stdout: File = File::create("/tmp/file_search_daemon.out").unwrap();
    // let stderr: File = File::create("/tmp/file_search_daemon.err").unwrap();
    //
    // let daemonize: Daemonize<()> = Daemonize::new()
    //     .stdout(stdout)
    //     .stderr(stderr)
    //     .pid_file("/tmp/file_search_daemon.pid")
    //     .chown_pid_file(true);
    //
    // match daemonize.start() {
    //     Ok(_) => println!("Daemon started successfully."),
    //     Err(e) => eprintln!("Error starting daemon: {}", e),
    // }

    let now = Instant::now();


    let config_content: String = match fs::read_to_string("/etc/file_search/config.toml"){
        Ok(conf_cont) => conf_cont,
        Err(err_msg) => panic!("/*************************************************************/\n\
                                      Could not read config.tom file due to: {}\n\
                                      /*************************************************************/",err_msg)
    };

    let config: Config = match toml::from_str(&config_content) {
        Ok(conf) => conf,
        Err(err_msg) => panic!("/*************************************************************/\n\
                                    Could not translate configuration params due to: {}\n\
                                    /*************************************************************/",err_msg)
    };

    // println!("Is hidden? : {:?}", config.setup_config.add_hidden_flag);
    // println!("Setup mode : {:?}", config.setup_config.setup_mode);
    // println!("Included dirs : {:?}", config.setup_config.included_dirs);

    let setup_mode: SetupKind = config.setup_config.setup_mode;
    let include_dirs: Vec<String> = config.setup_config.included_dirs;
    let add_hidden_flag: bool = config.setup_config.add_hidden_flag;

    delete_lib_dir();
    create_lib_dir();
    create_dbs();

    populate_db(setup_mode, include_dirs, add_hidden_flag);

    create_index_on_tables();
    println!("{}", now.elapsed().as_secs());
    println!("Database update is complete!");
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
                Err(err_msg) => panic!("/*************************************************************/\n\
                                            Database could not be created due to: {}\n\
                                            /*************************************************************/", err_msg),
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
            Err(err_msg) => panic!("/*************************************************************/\n\
                                        Directory could not be created due to: {}\n\
                                        /*************************************************************/", err_msg),
        }
    } else {
        println!("Directory {} already exists!", dir_path);
    }
}

fn delete_lib_dir() {
    let dir_path = "/var/lib/file_search";
    if Path::new(dir_path).exists() {
        match fs::remove_dir_all(dir_path) {
            Ok(_) => println!("Directory {} successfully deleted!", dir_path),
            Err(err_msg) => panic!("/*************************************************************/\n\
                                        Directory could not be deleted due to: {}\n\
                                        /*************************************************************/", err_msg),
        }
    } else {
        println!("Directory {} does not exist!", dir_path);
    }
}