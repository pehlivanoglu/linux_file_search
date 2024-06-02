use dialoguer::{theme::ColorfulTheme, Select};
use regex::Regex;
use rusqlite::{params, Connection};
use std::process::Command;
use std::str;
use clap::{App, Arg};

fn main() {
    let matches = App::new("search")
        .version("1.0")
        .about("Search and execute commands with optional directory updates")
        .arg(
            Arg::with_name("query")
                .help("Search query enclosed in double quotes")
                .required(true)
                .index(1),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let re = Regex::new(r"\[(.*?)\]").unwrap();

    let original_command = query
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut dirs_to_update: Vec<String> = Vec::new();
    let mut command_to_execute: Vec<String> = Vec::new();
    for arg in original_command.clone() {
        let filename: Result<String, String> = match re.captures(&arg) {
            Some(captures) => match captures.get(1) {
                Some(matched_text) => Ok(matched_text.as_str().to_string()),
                None => Err(String::from("No match found")),
            },
            None => Err(String::from("No match found")),
        };

        if let Ok(ref value) = filename {
            let search_result: Vec<String> = search(value);
            if search_result.len() == 1 {
                command_to_execute.push(handle_multiple_results(vec![search_result.get(0).unwrap().clone()]));
            } else if search_result.len() == 0 {
                let find_results: Vec<String> = search_w_find(value);
                if find_results.len() == 0 {
                    command_to_execute.push(arg.clone());
                } else if find_results.len() == 1 {
                    let dir: String = find_results.get(0).unwrap().clone();
                    command_to_execute.push(handle_multiple_results(vec![dir.clone()]));
                    dirs_to_update.push(dir.clone());
                } else {
                    let dir: String = handle_multiple_results(find_results.clone());
                    command_to_execute.push(dir.clone());
                    dirs_to_update.push(dir.clone());
                }
            } else {
                command_to_execute.push(handle_multiple_results(search_result.clone()));
            }
        } else {
            command_to_execute.push(arg.clone());
        }
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(command_to_execute.join(" "))
        .output()
        .unwrap_or_else(|e| {
            eprint!("Error executing command: {}", e);
            std::process::exit(1); // Exit the program with a non-zero status code
        });

    if !output.stdout.is_empty() {
        print!("\n{}\n", String::from_utf8_lossy(&output.stdout));
    } else {
        if !output.stderr.is_empty() {
            eprint!("\n{}\n", String::from_utf8_lossy(&output.stderr));
        }
    }

    add_files(dirs_to_update.clone());
}

fn add_files(dirs: Vec<String>) {

    for dir in dirs.clone() {
        let mut temp = dir
            .as_str()
            .split("/")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let filename = temp.get(temp.len() - 1).unwrap().to_string();
        let db_name = filename.chars().next().unwrap().to_string();
        let db_path = match filename.chars().next().unwrap().is_alphanumeric(){
            true => format!("/var/lib/file_search/{}.db", db_name),
            false => String::from("/var/lib/file_search/_.db")
        };
        temp.pop();
        let mut path = temp.join("/");
        path.push('/');

        let connection = Connection::open(db_path.clone()).unwrap();
        let stmt = connection.prepare("INSERT INTO files (path, filename) VALUES (?, ?)");

        match stmt {
            Ok(mut stmt) => {
                let res = stmt.execute(params![format!("{}{}",path,filename), filename]);
                // Handle the result of the SQL operation
                if let Err(err) = res {
                    println!("Error inserting into {}: {}: Database cannot be updated, run with sudo to update.", db_path, err);
                }
            }
            Err(e) => println!("Failed to prepare statement: {}", e),
        }
    }
}

fn handle_multiple_results(results: Vec<String>) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select a file")
        .items(&results)
        .default(0)
        .interact()
        .expect("Failed to create selection menu");

    return results[selection].clone();
}

fn search_w_find(filename: &str) -> Vec<String> {
    let excluded_dirs = vec!["/proc", "/run", "/lost+found", "/tmp", "/dev"];
    let mut command = Command::new("sudo");
    command.arg("find").arg("/");

    for dir in &excluded_dirs {
        command.arg("-path").arg(dir).arg("-prune").arg("-o");
    }

    command.arg("-name").arg(filename).arg("-print");

    let output = command.output().expect("failed to execute process");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("failed to parse output");
        let lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();
        lines
    } else {
        Vec::new()
    }
}

fn search(filename: &str) -> Vec<String> {
    let connection: Connection = open_sql_connection(filename);

    let sql_query: &str = "SELECT path, filename FROM files WHERE filename = ?1";

    let opt_stmt = connection.prepare(sql_query);

    let mut stmt = match opt_stmt {
        Ok(stmt) => stmt,
        Err(err) => panic!(
            "/*************************************************************/\n\
                                {}\n\
                                /*************************************************************/",
            err
        ),
    };

    let opt_rows = stmt.query(params![filename]);

    let mut rows = match opt_rows {
        Ok(rows) => rows,
        Err(err) => panic!(
            "/*************************************************************/\n\
                                {}\n\
                                /*************************************************************/",
            err
        ),
    };

    let mut files: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let path: String = row.get(0).unwrap();
        files.push(path.clone());
    }
    return files;
}

fn open_sql_connection(filename: &str) -> Connection {
    let first_char: char = filename.chars().nth(0).unwrap();
    let opt_conn = match first_char.is_alphanumeric() {
        false => Connection::open("/var/lib/file_search/_.db"),
        true => Connection::open(format!("/var/lib/file_search/{}.db", first_char)),
    };

    match opt_conn {
        Ok(c) => return c,
        Err(err) => {
            println!("ERR: {}: /var/lib/file_search/{}.db, run with sudo to update.",err ,first_char);
            std::process::exit(1);
        }
    }
}
