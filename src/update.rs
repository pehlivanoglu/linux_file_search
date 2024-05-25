use Utils::*;

//run with superuser permissions
fn main() {
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
