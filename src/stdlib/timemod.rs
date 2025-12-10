//! Time module - time and date operations
//!
//! This module provides functions for working with time, dates, and delays.

use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Get current Unix timestamp (seconds since epoch)
pub fn time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Get current Unix timestamp in milliseconds
pub fn time_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Sleep for a given number of seconds
pub fn sleep(seconds: f64) {
    std::thread::sleep(Duration::from_secs_f64(seconds));
}

/// Sleep for a given number of milliseconds
pub fn sleep_ms(milliseconds: i64) {
    std::thread::sleep(Duration::from_millis(milliseconds as u64));
}

/// Get the current system time as a string representation
pub fn now_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| format!("Timestamp: {}", d.as_secs()))
        .unwrap_or_else(|_| "Time error".to_string())
}

/// Measure elapsed time (returns time in seconds)
pub fn elapsed_time(start_time: i64) -> i64 {
    time() - start_time
}
