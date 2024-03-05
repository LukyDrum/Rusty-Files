mod file_manager;

use std::path::PathBuf;

use file_manager::Manager;

use crate::file_manager::Entry;

fn main() {
    let mut manager: Manager = Manager::new();

    println!("Before:");
    for entry in &manager.current_directory_entries {
        println!("{}", entry.filename());
    }
}