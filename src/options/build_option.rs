//! Command line options for the `build` subcommand
//!
//! This module defines parameters for the high-level build command that
//! combines compilation and packaging with intelligent defaults.

use crate::commands::STABLE_ABI_TYPE;
use crate::commands::UNSTABLE_ABI_TYPE;
use clap::Args;

/// Command line options for the `build` subcommand
///
/// This struct defines parameters for the high-level build command that
/// combines compilation and packaging with intelligent defaults.
#[derive(Args, Debug)]
pub struct BuildOptions {
    /// Path to Cargo.toml (defaults to current directory)
    #[arg(short = 'm', long, value_name = "MANIFEST-PATH")]
    pub manifest_path: Option<String>,

    /// Output extension file path (auto-detected if not specified)
    #[arg(short = 'o', long, value_name = "EXTENSION-PATH")]
    pub extension_path: Option<String>,

    /// Extension version (auto-detected from Cargo.toml if not specified)
    #[arg(short = 'v', long, value_name = "EXTENSION-VERSION")]
    pub extension_version: Option<String>,

    /// Target platform (auto-detected from build target if not specified)
    #[arg(short = 'p', long, value_name = "DUCKDB-PLATFORM")]
    pub duckdb_platform: Option<String>,

    /// DuckDB version (auto-detected from dependencies if not specified)
    #[arg(short = 'd', long, value_name = "DUCKDB-VERSION", conflicts_with = "duckdb_capi_version")]
    duckdb_version: Option<String>,

    /// DuckDB C API version the extension is built for (e.g., "v1.2.0")
    #[arg(short = 'a', long, value_name = "DUCKDB-CAPI-VERSION", conflicts_with = "duckdb_version")]
    duckdb_capi_version: Option<String>,

    /// Additional arguments passed to `cargo build`
    #[arg(raw = true)]
    pub args: Vec<String>,
}

impl BuildOptions {
    /// Returns the DuckDB version or DuckDB C API version
    ///
    /// This method provides the version string based on user input,
    /// preferring DuckDB version over C API version when both are present.
    pub fn version(&self) -> Option<String> {
        self.duckdb_version.as_ref()
            .or(self.duckdb_capi_version.as_ref())
            .map(|version| version.to_string())
    }

    /// Determines the ABI type based on version specification
    ///
    /// Returns `C_STRUCT` for C API versions (stable ABI) or
    /// `C_STRUCT_UNSTABLE` for regular DuckDB versions (unstable ABI).
    pub fn abi_type(&self) -> String {
        let abi_type = if self.duckdb_capi_version.is_some() {
            STABLE_ABI_TYPE
        } else {
            UNSTABLE_ABI_TYPE
        };
        abi_type.to_string()
    }
}
