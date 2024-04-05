//!
//! Logging Platform Abstraction for OpenThread
//!

use alloc::string::String;

/// Log levels 
pub enum OTLogLevel {
    No = 0,
    Critical = 1,
    Warn = 2,
    Note = 3,
    Info = 4,
    Debug = 5,
}

pub trait OTLogger {
    /// Output logs
    fn log(&mut self, log_level: OTLogLevel, message: String);
}