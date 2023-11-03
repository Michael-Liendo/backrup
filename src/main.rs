use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut backup_dir_path = String::new();
    let mut source_dir_path = String::new();
    let mut i = 0;

    // Parse the arguments
    for arg in &args {
        if arg == "-b" {
            if i + 1 >= args.len() {
                println!("Error: -b requires a directory to backup");
                std::process::exit(1);
            }
            backup_dir_path = args[i + 1].clone();
        }
        if arg == "-s" {
            if i + 1 >= args.len() {
                println!("Error: -s requires a directory to source");
                std::process::exit(1);
            }
            source_dir_path = args[i + 1].clone();
        }
        if arg == "-h" {
            println!("Usage:  -b <backup_dir> -s <source_dir>");
            std::process::exit(0);
        }
        i += 1;
    }

    // Check if the arguments are empty
    if backup_dir_path == "" {
        println!("Error: -b requires a directory to backup");
        std::process::exit(1);
    }
    if source_dir_path == "" {
        println!("Error: -s requires a directory to source");
        std::process::exit(1);
    }

    // Convert to absolute paths
    let backup_directory = fs::canonicalize(backup_dir_path).unwrap();
    let source_directory = fs::canonicalize(source_dir_path).unwrap();

    // Check if the paths are directories
    if !backup_directory.is_dir() {
        println!(
            "Error: the backup path is is not a directory, {}",
            backup_directory.display()
        );
        std::process::exit(1);
    }
    if !source_directory.is_dir() {
        println!(
            "Error: the source path is not a directory, {}",
            source_directory.display()
        );
        std::process::exit(1);
    }

    // Check if the paths are the same
    if source_directory == backup_directory {
        println!("Error: the source and backup paths are the same");
        std::process::exit(1);
    }

    println!("Watching {} for changes", source_directory.display());

    let mut current_time = SystemTime::now();

    let mut initial_files: HashSet<String> = load_files(source_directory.clone());

    loop {
        let metadata = fs::metadata(&source_directory).unwrap();
        let last_modified = metadata.modified().unwrap();
        if current_time < last_modified {
            current_time = SystemTime::now();

            let current_files = load_files(source_directory.clone());

            let current_files_clone = current_files.clone();

            let deleted_files: HashSet<String> = initial_files
                .difference(&current_files_clone)
                .cloned()
                .collect();

            let new_files: HashSet<String> =
                current_files.difference(&initial_files).cloned().collect();

            initial_files = current_files;

            if new_files.len() > 0 {
                for file_path in new_files {
                    copy_file(
                        fs::canonicalize(file_path).unwrap(),
                        backup_directory.clone(),
                    );
                }
            }
        }
    }
}

fn copy_file(from_path: PathBuf, to_directory: PathBuf) {
    let file_name = from_path.file_name().unwrap().to_str().unwrap();
    let to_path = to_directory.join(file_name);

    fs::copy(from_path, to_path).unwrap();
}

fn load_files(directory: PathBuf) -> HashSet<String> {
    let mut files: HashSet<String> = HashSet::new();

    for entry in fs::read_dir(&directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.extend(load_files(path.clone()));
        } else {
            files.insert(path.display().to_string());
        }
    }

    files
}
