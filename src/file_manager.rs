use std::fs::{DirEntry, Metadata, read_dir};
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;
use std::env::current_dir;


/// Represents the type of the entry, either File or Directory or Symbolic Link (symlink).
pub enum EntryType {
    File, Directory, Symlink
}

/// Represents a single file or directory. Includes detailed information about the entry.
pub struct Entry {
    pub path: PathBuf,
    pub size: u64, // In bytes
    pub entry_type: EntryType,
    pub last_modified: SystemTime,
    pub is_hidden: bool,
}

impl Entry {
    /// Construct Entry from DirEntry (std::fs::DirEntry)
    pub fn from(dir_entry: DirEntry, original_path: PathBuf) -> Entry {
        // Get name of the entry
        let name: String = dir_entry.file_name().into_string().unwrap();

        // Check if it is a hidden file/directory (if it starts with '.')
        let is_hidden: bool = name.starts_with('.');

        let mut full_path: PathBuf = original_path.clone();
        full_path.push(name);
        
        // Get metadata - TODO: Handle possible error
        let entry_metadata: Metadata = dir_entry.metadata().unwrap();

        // Get the type of the entry
        let entry_type: EntryType = {
            if entry_metadata.is_symlink() {
                EntryType::Symlink
            }
            else if entry_metadata.is_dir() {
                EntryType::Directory
            }
            else {
                EntryType::File
            }
        };

        // Get the SystemTime of last modification or set it to UNIX_EPOCH
        let last_modified: SystemTime = entry_metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

        return Entry {
            path: full_path,
            size: entry_metadata.len(),
            entry_type: entry_type,
            last_modified: last_modified,
            is_hidden: is_hidden,
        };
    }

    pub fn filename(&self) -> String {
        // It is safe to unwrap as we will never ask for the name of root directory
        self.path.file_name().unwrap().to_os_string().into_string().unwrap()
    }
}

/// Represents the file manager backend.
pub struct Manager {
    pub current_directory: PathBuf,
    pub current_directory_entries: Vec<Entry>,
}

impl Manager {
    /// Constructs a new Manager instance with entries of the current directory
    pub fn new() -> Manager {
        // Get current directory, TODO: Handle errors
        let cur_dir: PathBuf = current_dir().unwrap();
        // Read the current directory and convert each DirEntry to Entry
        let entries: Vec<Entry> = read_dir(cur_dir.as_path()).unwrap().map(
            |dir_entry: Result<DirEntry, std::io::Error>| Entry::from(dir_entry.unwrap(), cur_dir.clone())
        ).collect();

        return Manager {
            current_directory: cur_dir,
            current_directory_entries: entries,
        };

    }

    // Changes the current directory and sets new directory entries
    // pub fn change_directory(&mut self, new_directory: PathBuf) -> Result<(), io::Error> {
        
    // }
}