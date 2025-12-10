//! I/O module - input/output operations
//!
//! This module provides functions for reading from and writing to files and streams.

use std::fs;
use std::io::{self, Write};

/// Read an entire file into a string
pub fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Write a string to a file
pub fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Append content to a file
pub fn append_file(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(content.as_bytes())
}

/// Read from stdin
pub fn read_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_string())
}

/// Write to stdout
pub fn write_output(msg: &str) -> Result<(), io::Error> {
    io::stdout().write_all(msg.as_bytes())?;
    io::stdout().flush()
}

/// Check if a file exists
pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/// Get the size of a file in bytes
pub fn file_size(path: &str) -> Result<u64, io::Error> {
    Ok(fs::metadata(path)?.len())
}

/// Read lines from a file
pub fn read_lines(path: &str) -> Result<Vec<String>, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

/// Write lines to a file (one per line)
pub fn write_lines(path: &str, lines: Vec<&str>) -> Result<(), io::Error> {
    fs::write(path, lines.join("\n"))
}
