//! Fancy progress bar functionality.

use crate::build::emoji;
use anyhow::{bail, Error, Result};
use console::style;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
/// The maximum log level
// The ordering is important: the least verbose must be at
// the top and the most verbose at the bottom
pub enum LogLevel {
    /// Logs only error
    Error,
    /// Logs only warn and error
    Warn,
    /// Logs everything
    Info,
}

impl std::str::FromStr for LogLevel {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            _ => bail!("Unknown log-level: {}", s),
        }
    }
}

/// Synchronized progress bar and status message printing.
pub struct ProgressOutput {
    quiet: AtomicBool,
    log_level: AtomicU8,
}

impl ProgressOutput {
    /// Returns a new ProgressOutput
    pub const fn new() -> Self {
        Self {
            quiet: AtomicBool::new(false),
            log_level: AtomicU8::new(LogLevel::Info as u8),
        }
    }

    /// Print the given message.
    fn message(&self, message: &str) {
        eprintln!("{}", message);
    }

    /// Returns whether it should silence stdout or not
    pub fn quiet(&self) -> bool {
        self.quiet.load(Ordering::SeqCst)
    }

    /// Returns whether the specified log level is enabled or not
    pub fn is_log_enabled(&self, level: LogLevel) -> bool {
        (level as u8) <= self.log_level.load(Ordering::SeqCst)
    }

    /// Add an informational message.
    pub fn info(&self, message: &str) {
        if !self.quiet() && self.is_log_enabled(LogLevel::Info) {
            let info = format!("{}: {}", style("[INFO]").bold().dim(), message,);
            self.message(&info);
        }
    }

    /// Add a warning message.
    pub fn warn(&self, message: &str) {
        if !self.quiet() && self.is_log_enabled(LogLevel::Warn) {
            let warn = format!(
                "{}: {} {}",
                style("[WARN]").bold().dim(),
                emoji::WARN,
                message
            );
            self.message(&warn);
        }
    }
}

impl Default for ProgressOutput {
    fn default() -> Self {
        ProgressOutput::new()
    }
}
