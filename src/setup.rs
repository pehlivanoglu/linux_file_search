use clap::{App, Arg};
use Utils::*;

//run with superuser permissions
fn main() {
    let mut setup_mode: SetupKind = SetupKind::Minimal;
    let mut include_dirs: Vec<&str> = Vec::new();
    let mut add_hidden_flag: bool = false;

    let app = App::new("file_search setup")
        .version("0.1")
        // .author("Ahmet Pehlivanoglu ahmet.pehlivanoglu@ozu.edu.tr")
        .about("Setup for file_search module")
        .arg(
            Arg::new("setup_mode")
                .short('s')
                .long("setup_mode")
                .takes_value(true)
                .default_value("minimal"),
        )
        .arg(
            Arg::new("include")
                .short('i')
                .long("include")
                .value_name("folder")
                .help("Includes files specified")
                .multiple_values(true) // Allows multiple values
                .takes_value(true),
        )
        .arg(
            Arg::new("add_hidden")
                .short('h')
                .long("add_hidden")
                .help("add hidden folders/files")
                .takes_value(false),
        )
        .get_matches();

    if app.is_present("add_hidden") {
        add_hidden_flag = true;
    }

    if let Some(values) = app.values_of("include") {
        include_dirs = values.collect::<Vec<_>>();
        println!("Considering hidden folders/files as well");
    }
    println!("Included folders: {}", include_dirs.join(", "));

    if app.is_present("setup_mode") {
        setup_mode = match app.value_of("setup_mode").unwrap().to_lowercase().as_str() {
            "default" => SetupKind::Default,
            "minimal" => SetupKind::Minimal,
            "standard" => SetupKind::Standard,
            "maximal" => SetupKind::Maximal,
            _ => panic!("Setup mode type is invalid"),
        }
    };
    println!("Setup mode: {}", app.value_of("setup_mode").unwrap());

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
