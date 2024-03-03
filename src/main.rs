mod file_manager;

use std::path::PathBuf;

use file_manager::Manager;

fn main() {
    let mut manager: Manager = Manager::new();

    for entry in &manager.current_directory_entries {
        println!("{}", entry.filename());
    }

    println!("");

    manager.change_directory(PathBuf::from("src")).unwrap();
    for entry in &manager.current_directory_entries {
        println!("{}", entry.filename());
    }
}