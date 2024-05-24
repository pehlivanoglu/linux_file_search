use Utils::*;

//run with superuser permissions
fn main() {
    let mut setup_mode: SetupKind = SetupKind::Minimal;
    let mut include_dirs: Vec<&str> = Vec::new();

    create_lib_dir();
    create_dbs();

    if setup_mode == SetupKind::Default {
        // println!("default");
        populate_db(setup_mode, include_dirs, add_hidden_flag);
    } else {
        // println!("other");
        populate_db(setup_mode, include_dirs, add_hidden_flag);
    }

    create_index_on_tables();
    println!("Database setup is complete!");
}
