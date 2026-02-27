//! Logging system for cargo-duckdb-ext
//!
//! This module provides a simple logging system with support for quiet mode,
//! allowing users to suppress console output when desired.
//! The log output format is compatible with Cargo's native commands
//! (12-character action prefix followed by message) and uses the same colors.

use anstyle::AnsiColor;
use anstyle::Color;
use anstyle::Style;
use std::fmt::Arguments;
use std::fmt::Display;
use std::sync::OnceLock;

const BOLD: Style = Style::new().bold();
const GREEN: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightGreen)));

/// Log action types for consistent output formatting
///
/// Each action corresponds to a specific phase of extension development
/// and is displayed with a 12-character right-aligned prefix.
pub(crate) enum Action {
    /// Creating new projects or files
    Creating,
    /// Copying files during packaging
    Copying,
    /// Building Rust code with cargo
    Building,
    /// Packaging extensions with metadata
    Packing,
    /// Completion of operations
    Finished,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:>12}", match *self {
            Action::Creating => "Creating",
            Action::Copying => "Copying",
            Action::Building => "Building",
            Action::Packing => "Packing",
            Action::Finished => "Finished",
        })
    }
}

/// Logger instance that controls console output
///
/// The logger can be disabled via the quiet flag to suppress
/// all console output while maintaining internal functionality.
#[derive(Clone, Debug)]
pub(crate) struct Logger {
    /// Whether logging is enabled
    enable: bool,
}

impl Logger {
    /// Logs a message with the specified action prefix
    ///
    /// # Arguments
    /// * `action` - The action type for formatting
    /// * `message` - The message content to display
    pub(crate) fn log(&self, action: Action, message: Arguments) {
        if self.enable {
            anstream::eprintln!("{BOLD}{GREEN}{action}{GREEN:#}{BOLD:#} {message}");
        }
    }
}

pub(crate) static LOGGER: OnceLock<Logger> = OnceLock::new();

/// Initializes the global logger with the specified quiet mode setting
///
/// # Arguments
/// * `quiet` - If true, suppresses all console output
pub(crate) fn init(quiet: bool) {
    LOGGER
        .set(Logger { enable: !quiet })
        .expect("Could not initialize logger");
}

/// Checks if logging is currently enabled
///
/// # Returns
/// `true` if logging is enabled, `false` otherwise
pub(crate) fn enabled() -> bool {
    LOGGER.get().map(|logger| logger.enable).unwrap_or(false)
}

/// Logs a message with the "Creating" action prefix
///
/// This macro is used when creating new projects or files.
#[macro_export]
macro_rules! creating {
    ($($messages:tt)*) => {{
        if let Some(logger) = logging::LOGGER.get() {
            logger.log(logging::Action::Creating, format_args!($($messages)*));
        }
    }};
}

/// Logs a message with the "Copying" action prefix
///
/// This macro is used during file copying operations in packaging.
#[macro_export]
macro_rules! copying {
    ($($messages:tt)*) => {{
        if let Some(logger) = logging::LOGGER.get() {
            logger.log(logging::Action::Copying, format_args!($($messages)*));
        }
    }};
}

/// Logs a message with the "Building" action prefix
///
/// This macro is used during cargo build operations.
#[macro_export]
macro_rules! building {
    ($($messages:tt)*) => {{
        if let Some(logger) = logging::LOGGER.get() {
            logger.log(logging::Action::Building, format_args!($($messages)*));
        }
    }};
}

/// Logs a message with the "Packing" action prefix
///
/// This macro is used during extension packaging operations.
#[macro_export]
macro_rules! packing {
    ($($messages:tt)*) => {{
        if let Some(logger) = logging::LOGGER.get() {
            logger.log(logging::Action::Packing, format_args!($($messages)*));
        }
    }};
}

/// Logs a message with the "Finished" action prefix
///
/// This macro is used to indicate completion of operations.
#[macro_export]
macro_rules! finished {
    ($($messages:tt)*) => {{
        if let Some(logger) = logging::LOGGER.get() {
            logger.log(logging::Action::Finished, format_args!($($messages)*));
        }
    }};
}
