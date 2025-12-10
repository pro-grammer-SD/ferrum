//! System module - system information and utilities
//!
//! This module provides functions to access system information and control program flow.

use std::env;
use std::process;

/// Get command-line arguments
pub fn argv() -> Vec<String> {
    env::args().collect()
}

/// Get the program name (argv[0])
pub fn program_name() -> String {
    env::args()
        .next()
        .unwrap_or_else(|| "ferrum".to_string())
}

/// Exit the program with a status code
pub fn exit(code: i32) -> ! {
    process::exit(code)
}

/// Get the Ferrum version
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get all environment variables as a list of tuples
pub fn environ() -> Vec<(String, String)> {
    env::vars().collect()
}

/// Get the system architecture
pub fn architecture() -> &'static str {
    env::consts::ARCH
}

/// Get the target OS
pub fn os() -> &'static str {
    env::consts::OS
}

/// Check if running on Windows
pub fn is_windows() -> bool {
    cfg!(windows)
}

/// Check if running on Linux
pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

/// Check if running on macOS
pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}
