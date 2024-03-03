mod file_manager;

use file_manager::Manager;

fn main() {
    let manager: Manager = Manager::new();

    for entry in &manager.current_directory_entries {
        println!("{}", entry.filename());
    }
}