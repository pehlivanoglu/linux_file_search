use std::env;
use std::panic::panic_any;
use std::path::Path;
use rusqlite::{params, Connection};
use std::time::Instant;


fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let filename: &str = args.get(1).unwrap();

    // for arg in &args{
    //     println!("{}", arg);
    // }
    // println!();

    match args.get(2) {
        Some(p) => println!("path detected: {}/{}",p,filename)/*search_w_path(&filename, p)*/,
        None => search_wo_path(&filename),
    }
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}

fn search_wo_path(filename: &str){
    // if !is_file(&filename) {
    //     panic!("{} is not a file!", &filename);
    // }
    let connection: Connection = open_sql_connection(filename);

    let sql: &str = "SELECT path, filename FROM files WHERE filename = ?1";

    let mut opt_stmt = connection.prepare(sql);

    let mut stmt = match opt_stmt {
            Ok(stmt) => stmt,
            Err(err) => panic!("{}asdasd;ksa;kd",err),
        };

    let opt_rows = stmt.query(params![filename]);

    let mut rows = match opt_rows {
        Ok(rows) => rows,
        _ => panic!("fgh"),
    };

    println!("geldim");
    while let Some(row) = rows.next().unwrap() {
        let path: String = row.get(0).unwrap();
        let filename: String = row.get(1).unwrap();
        println!("path = {}", path);
        println!("file = {}", filename);
    }
    // let stmt = connection.prepare("SELECT *\
    //                                                     FROM files \
    //                                                      WHERE filename = ?");
    //
    // let res = match stmt {
    //     Ok(mut stmt) => stmt.execute(params![filename]),
    //     _ => Ok(1),
    // };
    //
    // println!("path = {}", stmt.read::<String, _>("path").unwrap());
    // println!("file = {}", stmt.read::<String, _>("filename").unwrap());
}

fn search_w_path(filename: &str, path: &str){
    if !is_file(&filename) {
        panic!("{} is not a file!", &filename);
    }
    if !is_valid_linux_directory(&path) {
        panic!("{} is not a directory!", &path);
    }
    // let first_char: char = filename.chars().nth(0).unwrap();
}

fn open_sql_connection(filename: &str) -> Connection{
    let first_char: char = filename.chars().nth(0).unwrap().to_ascii_lowercase();
    let opt_conn = match first_char {
        '.' => Connection::open("/var/lib/file_search/.db"),
        _ => Connection::open(format!("/var/lib/file_search/{}.db", first_char)),
    };

    match opt_conn{
        Ok(c) => return c,
        Err(err) => panic!("{}",err),
    }
}

fn is_file(s: &str) -> bool {
    match s.rfind('.') {
        Some(index) => {
            index + 1 < s.len()
        }
        None => false,
    }
}

fn is_valid_linux_directory(path: &str) -> bool {
    let path = Path::new(path);
    path.is_absolute() && path.to_str().is_some() && !path.to_str().unwrap().contains('\0')
}




