use std::fs::{DirEntry, Metadata, read_dir, rename};
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::time::SystemTime;
use std::env::current_dir;


type IOError = Error;

/// Represents the type of the entry, either File or Directory or Symbolic Link (symlink).
#[derive(Clone)]
pub enum EntryType {
    File, Directory, Symlink
}

/// Represents a single file or directory. Includes detailed information about the entry.
#[derive(Clone)]
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

    /// Get the filename of this entry
    pub fn filename(&self) -> String {
        // It is safe to unwrap as we will never ask for the name of root directory
        self.path.file_name().unwrap().to_os_string().into_string().unwrap()
    }

    /// Rename this Entry
    pub fn rename(&mut self, new_name: String) -> Result<(), IOError> {
        // Get new full path - it is safe to unwrap as we will never try to rename a root directory
        let new_path: PathBuf = self.path.parent().unwrap().join(PathBuf::from(new_name));
        // Rename the file/directory
        let rename_result = rename(&self.path, &new_path);
        match rename_result {
            Ok(_) => {
                self.path = new_path;
                return Ok(());
            },
            Err(err) => return Err(err)
        };
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
        let entries: Vec<Entry> = Manager::get_entries_in_directory(cur_dir.clone());

        return Manager {
            current_directory: cur_dir,
            current_directory_entries: entries,
        };

    }

    /// Construct a new Manager instance with entries and current directory set to the argument path
    pub fn with_directory(directory: PathBuf) -> Result<Manager, IOError> {
        let mut manager = Manager {
            current_directory: PathBuf::new(),
            current_directory_entries: Vec::new(),
        };

        let result = manager.change_directory(directory);
        match result {
            Ok(_) => return Ok(manager),
            Err(err) => return Err(err),
        };
    }

    /// Changes the current directory and sets new directory entries
    pub fn change_directory(&mut self, new_directory: PathBuf) -> Result<(), IOError> {
        // Get the full path of the directory
        let full_new_directory_path = {
            if new_directory.is_absolute() {
                new_directory
            }
            else {
                self.current_directory.join(new_directory)
            }
        };

        // Check if the path is really a directory
        if full_new_directory_path.is_dir() {
            self.current_directory_entries = Manager::get_entries_in_directory(full_new_directory_path);

            return Ok(());
        }
        else {
            return Err(IOError::new(ErrorKind::NotFound, "Directory not found!"));
        }
    }

    /// Moves to the parent directory
    pub fn move_to_parent_directory(&mut self) -> () {
        let parent_directory_opt = self.current_directory.parent();
        match parent_directory_opt {
            Some(dir) => self.change_directory(dir.to_path_buf()).unwrap(), // TODO: Handle Error
            None => {}
        };
    }

    /// Returns a vector of Entry(ies) in the specified directory
    fn get_entries_in_directory(directory: PathBuf) -> Vec<Entry> {
        // Read the directory and convert each DirEntry to Entry
        read_dir(directory.as_path()).unwrap().map(
            |dir_entry: Result<DirEntry, std::io::Error>| Entry::from(dir_entry.unwrap(), directory.clone())
        ).collect()
    }
}

