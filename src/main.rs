use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let blueprint_dir = PathBuf::from(format!("{}/.blueprint", env::var("HOME").unwrap()));
    let current_dir = env::current_dir().unwrap();
    if env::args().len() == 1 {
        help();
        return;
    }
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-l" | "list" | "ls" => list(&blueprint_dir),
            "-c" | "create" => {
                if let Some(blueprint_name) = args.next() {
                    create(&blueprint_dir, &current_dir, &blueprint_name);
                } else {
                    println!("Missing blueprint name");
                    std::process::exit(1);
                }
            }
            "-n" | "new" => {
                if let Some(blueprint_name) = args.next() {
                    new(&blueprint_dir, &current_dir, &blueprint_name);
                } else {
                    println!("Missing blueprint name");
                    std::process::exit(1);
                }
            }
            "-r " | "remove" => {
                if let Some(blueprint_name) = args.next() {
                    remove(&blueprint_dir, &blueprint_name);
                } else {
                    println!("Missing blueprint name");
                    std::process::exit(1);
                }
            }
            "-h" | "--help" | "help" => help(),
            _ => println!("Unknown argument {}", arg),
        }
    }
}

fn help() {
    println!("Usage: blueprint [OPTIONS] [SUBCOMMAND]");
    println!("");
    println!("Options:");
    println!("  -h, --help       Prints help information");
    println!("");
    println!("Subcommands:");
    println!("  create <name>    make a new file from blueprint from an existing blueprint");
    println!("  list             List all blueprints");
    println!("  new <name>       Create a new blueprint from the current directory");
    println!("  remove <name>    Remove a blueprint");
}

/// list all blueprints
/// # Arguments
/// * `blueprint_dir` - the directory where all blueprints are stored
fn list(blueprint_dir: &PathBuf) {
    for entry in fs::read_dir(blueprint_dir).unwrap() {
        let entry = entry.unwrap();
        println!("{}", entry.file_name().into_string().unwrap());
    }
}

/// create a new blueprint from the current directory
/// # Arguments
/// * `blueprint_dir` - the directory where all blueprints are stored
/// * `current_dir` - the current directory
/// * `blueprint_name` - the name of the blueprint to create
fn new(blueprint_dir: &PathBuf, current_dir: &PathBuf, blueprint_name: &str) {
    let blueprint_path = blueprint_dir.join(blueprint_name);
    if blueprint_path.exists() {
        println!("Blueprint {} already exists", blueprint_name);
        std::process::exit(1);
    }
    fs::create_dir(&blueprint_path).unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let entry_name = entry.file_name().into_string().unwrap();
        let blueprint_file_path = blueprint_path.join(entry_name);
        fs::copy(entry.path(), blueprint_file_path).unwrap();
    }
}

/// create a blueprint from an existing blueprint
/// # Arguments
/// * `blueprint_dir` - the directory where all blueprints are stored
/// * `current_dir` - the current directory
/// * `blueprint_name` - the name of the blueprint to create
fn create(blueprint_dir: &PathBuf, current_dir: &PathBuf, blueprint_name: &str) {
    let blueprint_path = blueprint_dir.join(blueprint_name);
    if !blueprint_path.exists() {
        println!("Blueprint {} does not exist", blueprint_name);
        std::process::exit(1);
    }
    for entry in fs::read_dir(&blueprint_path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            continue;
        }
        let entry_name = entry.file_name().into_string().unwrap();
        let current_file_path = current_dir.join(entry_name);
        fs::copy(entry_path, current_file_path).unwrap();
    }
}

/// remove a blueprint
/// # Arguments
/// * `blueprint_dir` - the directory where all blueprints are stored
/// * `blueprint_name` - the name of the blueprint to remove
fn remove(blueprint_dir: &PathBuf, blueprint_name: &str) {
    let blueprint_path = blueprint_dir.join(blueprint_name);
    if !blueprint_path.exists() {
        println!("Blueprint {} does not exist", blueprint_name);
        std::process::exit(1);
    }
    fs::remove_dir_all(blueprint_path).unwrap();
}

