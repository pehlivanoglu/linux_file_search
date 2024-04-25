use rusqlite::{params, Connection};

fn main(){
    for c in 'a'..='z' {
        let path: String = format!("/var/lib/file_search/{}.db", c);
        let conn = Connection::open(path).unwrap();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_filename ON files(filename);",
            params![],
        ).unwrap();
    }
    for c in 'A'..='Z' {
        let path: String = format!("/var/lib/file_search/{}.db", c);
        let conn = Connection::open(path).unwrap();

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_filename ON files(filename);",
            params![],
        ).unwrap();
    }
    let path: String = String::from("/var/lib/file_search/_.db");
    let conn = Connection::open(path).unwrap();

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_filename ON files(filename);",
        params![],
    ).unwrap();

}