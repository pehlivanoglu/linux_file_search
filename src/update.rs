use std::fs::{self, File};
use std::time::Instant;
use daemonize::Daemonize;

use file_search_lib::{Config, create_index_on_tables, create_dbs_util, populate_db, create_lib_dir, delete_lib_dir};

fn main() {
    let stdout = File::create("/tmp/file_search_daemon.out").unwrap();
    let stderr = File::create("/tmp/file_search_daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .stdout(stdout)
        .stderr(stderr)
        .pid_file("/tmp/file_search_daemon.pid")
        .chown_pid_file(true);

    if let Err(e) = daemonize.start() {
        eprintln!("Error starting daemon: {}", e);
        return;
    }

    let now = Instant::now();

    let config_content = fs::read_to_string("/etc/file_search/config.toml")
        .expect("Could not read config.tom file");

    let config: Config = toml::from_str(&config_content)
        .expect("Could not translate configuration params");

    let setup_mode = config.setup_config.setup_mode.clone();
    let include_dirs = config.setup_config.included_dirs.clone();
    let add_hidden_flag = config.setup_config.add_hidden_flag;

    delete_lib_dir();
    create_lib_dir();
    create_dbs();
    populate_db(setup_mode, include_dirs, add_hidden_flag);
    create_index_on_tables();

    println!("{} seconds elapsed", now.elapsed().as_secs());
    println!("Database update is complete!");
}

fn create_dbs() {
    create_dbs_util('a', 'z');
    create_dbs_util('A', 'Z');
    create_dbs_util('0', '9');
    create_dbs_util('_', '_');
}
