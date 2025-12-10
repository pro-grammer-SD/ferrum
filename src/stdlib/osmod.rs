//! Operating system module - OS-level operations
//!
//! This module provides functions for interacting with the operating system,
//! such as file/directory operations, environment variables, and process control.

use std::fs;
use std::path::Path;
use std::env;

/// List files and directories in a directory
pub fn listdir(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut entries = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            entries.push(name.to_string());
        }
    }
    Ok(entries)
}

/// Get current working directory
pub fn getcwd() -> String {
    env::current_dir()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_else(|| ".".to_string())
}

/// Change current working directory
pub fn chdir(path: &str) -> Result<(), std::io::Error> {
    env::set_current_dir(path)
}

/// Check if a path is a directory
pub fn isdir(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// Check if a path is a file
pub fn isfile(path: &str) -> bool {
    Path::new(path).is_file()
}

/// Create a directory
pub fn mkdir(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir(path)
}

/// Create a directory and all missing parent directories
pub fn makedirs(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)
}

/// Remove a file
pub fn remove_file(path: &str) -> Result<(), std::io::Error> {
    fs::remove_file(path)
}

/// Remove a directory
pub fn remove_dir(path: &str) -> Result<(), std::io::Error> {
    fs::remove_dir(path)
}

/// Remove a directory and all its contents
pub fn remove_dir_recursive(path: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path)
}

/// Get an environment variable
pub fn getenv(name: &str) -> Option<String> {
    env::var(name).ok()
}

/// Set an environment variable
pub fn setenv(name: &str, value: &str) {
    env::set_var(name, value);
}

/// Get the name of the operating system
pub fn platform() -> &'static str {
    env::consts::OS
}

/// Get the current username
pub fn get_username() -> Option<String> {
    env::var("USER").ok()
        .or_else(|| env::var("USERNAME").ok())
}
