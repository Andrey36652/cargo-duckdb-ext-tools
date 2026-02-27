//! File system operations for cargo-duckdb-ext-tools
//!
//! This module provides file system utilities specifically designed for
//! creating DuckDB extension files by duplicating dynamic libraries and
//! preparing them for metadata appending.

use crate::copying;
use crate::logging;
use cargo_metadata::camino::Utf8PathBuf;
use std::fs::copy;
use std::fs::File;
use std::fs::OpenOptions;

/// Creates a duplicate of a file and opens it in append mode
///
/// This function is used to create the extension file by copying the
/// original dynamic library and then opening it for metadata appending.
///
/// # Arguments
/// * `source` - Path to the source dynamic library file
/// * `target` - Path where the extension file should be created
///
/// # Returns
/// A `File` handle opened in append mode for writing metadata
pub(crate) fn open_duplicate(source: &str, target: &str) -> Result<File, std::io::Error> {
    copying!("Library File ({source})");
    copying!("Extension File ({target})");
    copy(source, target)?;
    OpenOptions::new().append(true).open(target)
}

/// Trait for normalizing file system paths
///
/// This trait provides a method to convert platform-specific path
/// representations into a consistent string format, handling Windows
/// verbatim path prefixes.
pub(crate) trait NormalizedPath {
    /// Converts the path to a normalized string representation
    ///
    /// On Windows, this removes the `\\?\` verbatim path prefix if present.
    /// On other platforms, returns the path as-is.
    fn normalize(&self) -> String;
}

impl NormalizedPath for Utf8PathBuf {
    fn normalize(&self) -> String {
        let path = self.as_str();
        if cfg!(windows) && let Some(stripped) = path.strip_prefix("\\\\?\\") {
            stripped.to_owned()
        } else {
            path.to_owned()
        }
    }
}
