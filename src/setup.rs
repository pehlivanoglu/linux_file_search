use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

//run with superuser permissions
fn main() {
    create_lib_dir();
    create_dbs();
    populate_db();
    // delete_empty_dbs();
    println!("Database setup is complete!");

}

fn delete_empty_dbs(){

}

fn create_dbs_util(start: char, end:char){
    for c in start..=end {
        let path:String = format!("/var/lib/file_search/{}.db",c);
        if !Path::new(&path).exists() {
            let file_result = fs::File::create(&path);
            match file_result {
                Ok(_) => println!("Database {} successfully created!", &path),
                Err(err_msg) => panic!("Database could not be created due to: {}", err_msg),
            }
            let connection = Connection::open(&path).unwrap();
            connection.execute(
                "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            filename TEXT NOT NULL
        )", [],
            ).unwrap();
        }
    }
}

fn create_dbs() {
    create_dbs_util('a','z');
    create_dbs_util('A','Z');
    create_dbs_util('0','9');
}

fn populate_db() {
    let mut count: i128 = 0;
    for entry in WalkDir::new("/")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().starts_with("/proc") && !e.path().starts_with("/run") && !e.path().starts_with("/lost+found"))
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if let Some(first_char) = filename.chars().next() {
                    let first_char = first_char.to_lowercase().next().unwrap();
                        if first_char.is_alphabetic() {
                            let db_path = format!("/var/lib/file_search/{}.db", first_char);
                            count+=1;
                            let connection = Connection::open(&db_path).unwrap();
                            let stmt = connection.prepare("INSERT INTO files (path, filename) VALUES (?, ?)");
                            match stmt {
                                Ok(mut stmt) =>
                                    if db_path == connection.path().unwrap().to_str().unwrap() {
                                    let res = &stmt.execute(params![path.to_str(), filename]);
                                    if let Err(err_msg) = res {
                                        println!("{}", err_msg);
                                        print!("{}",count);
                                    }
                                },
                                Err(_) => continue,
                            }

                        }
                }
            }
        }
    }
    print!("{}",count);
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