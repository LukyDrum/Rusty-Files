mod file_manager;

use std::path::PathBuf;

use file_manager::Manager;

fn main() {
    let manager: Manager = Manager::with_directory(PathBuf::from("/home/lukydrum/")).unwrap();

    for entry in &manager.current_directory_entries {
        println!("{}", entry.filename());
    }
}