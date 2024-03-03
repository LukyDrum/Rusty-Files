mod file_manager;

use std::path::PathBuf;

use file_manager::Manager;

fn main() {
    let mut manager: Manager = Manager::new();

    for entry in &manager.current_directory_entries {
        println!("{}", entry.filename());
    }

    println!("");

    manager.move_to_parent_directory();
    for entry in &manager.current_directory_entries {
        println!("{}", entry.filename());
    }
}