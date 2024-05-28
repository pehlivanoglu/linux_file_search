mod update;

use rusqlite::{params, Connection};
use std::env;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = args.get(1).unwrap();

    // let start = Instant::now();

    search_wo_path(&filename);

    // let duration = start.elapsed();
    //
    // println!("{:?}", duration);
    // println!("Search done.");
}

fn search_wo_path(filename: &str) {
    let connection: Connection = open_sql_connection(filename);

    let sql_query: &str = "SELECT path, filename FROM files WHERE filename = ?1";

    let opt_stmt = connection.prepare(sql_query);

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
        println!("{}", path);
    }
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
