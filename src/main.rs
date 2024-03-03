mod file_manager;

use file_manager::Manager;

fn main() {
    let manager: Manager = Manager::new();
    println!("{:?}", manager.current_directory);
}