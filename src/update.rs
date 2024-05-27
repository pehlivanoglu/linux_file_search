use Utils::*;
use daemonize::Daemonize;
use std::fs::File;

//run with superuser permissions
fn main() {

    // let stdout = match File::create("/tmp/file_search_daemon.out"){
    //     Ok(file) => file,
    //     Err(err_msg) => panic!("Could not open daemon stdout file: {}", err_msg)
    // };
    //
    // let stderr= match File::create("/tmp/file_search_daemon.err"){
    //     Ok(file) => file,
    //     Err(err_msg) => panic!("Could not open daemon stderr file: {}", err_msg)
    // };
    //
    // let daemon: Daemonize<()> = Daemonize::new()
    //     .stdout(stdout)
    //     .stderr(stderr)
    //     .pid_file("/tmp/file_search_daemon.pid")
    //     .chown_pid_file(true);
    //
    // match daemon.start() {
    //     Ok(_) => (),
    //     Err(err_msg) => panic!("Error starting file_search daemon: {}", err_msg)
    // }

    let binding = read_config_file();
    let include_dirs = binding.0.iter().map(|s| s.as_str()).collect();
    let setup_mode = binding.1;
    let add_hidden_flag = binding.2;
    delete_lib_dir();
    create_lib_dir();
    create_dbs();

    populate_db(setup_mode, include_dirs, add_hidden_flag);

    create_index_on_tables();
    println!("Database setup is complete!");
}
