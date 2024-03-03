mod file_manager;

use file_manager::Manager;

fn main() {
    let manager: Manager = Manager::new();
    println!("{:?}", manager.current_directory);
    for entry in &manager.current_directory_entries {
        println!("{} {}B", entry.name, entry.size);
    }
}