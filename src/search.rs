mod update;

use rusqlite::{params, Connection};
use std::env;
use std::path::Path;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename: &str = args.get(1).unwrap();

    for arg in &args{
        println!("{}", arg);
    }
    println!();
    let start = Instant::now();
    match args.get(2) {
        Some(p) => println!("path detected: {}/{}", p, filename) /*search_w_path(&filename, p)*/,
        None => search_wo_path(&filename),
    }
    let duration = start.elapsed();

    println!("{:?}", duration);
    println!("Search done.");
}

fn search_wo_path(filename: &str) {
    // if !is_file(&filename) {
    //     panic!("{} is not a file!", &filename);
    // }
    let connection: Connection = open_sql_connection(filename);

    let sql: &str = "SELECT path, filename FROM files WHERE filename = ?1";

    let opt_stmt = connection.prepare(sql);

    let mut stmt = match opt_stmt {
        Ok(stmt) => stmt,
        Err(err) => panic!("{}", err),
    };

    let opt_rows = stmt.query(params![filename]);

    let mut rows = match opt_rows {
        Ok(rows) => rows,
        Err(err) => panic!("{}", err),
    };


    while let Some(row) = rows.next().unwrap() {
        let path: String = row.get(0).unwrap();
        let filename: String = row.get(1).unwrap();
        println!("{}/{}", path,filename);
    }
}

fn search_w_path(filename: &str, path: &str) {
    if !is_file(&filename) {
        panic!("{} is not a file!", &filename);
    }
    if !is_valid_linux_directory(&path) {
        panic!("{} is not a directory!", &path);
    }
    // let first_char: char = filename.chars().nth(0).unwrap();
}

fn open_sql_connection(filename: &str) -> Connection {
    let first_char: char = filename.chars().nth(0).unwrap();
    let opt_conn = match first_char.is_alphanumeric() {
        false => Connection::open("/var/lib/file_search/_.db"),
        true => Connection::open(format!("/var/lib/file_search/{}.db", first_char)),
    };

    match opt_conn {
        Ok(c) => return c,
        Err(err) => panic!("{}", err),
    }
}

fn is_file(s: &str) -> bool {
    match s.rfind('.') {
        Some(index) => index + 1 < s.len(),
        None => false,
    }
}

fn is_valid_linux_directory(path: &str) -> bool {
    let path = Path::new(path);
    path.is_absolute() && path.to_str().is_some() && !path.to_str().unwrap().contains('\0')
}
