//! Command line options for the `package` subcommand
//!
//! This module defines all the parameters required to append DuckDB
//! extension metadata to an existing dynamic library file.

use crate::commands::STABLE_ABI_TYPE;
use crate::commands::UNSTABLE_ABI_TYPE;
use clap::Args;

/// Command line options for the `package` subcommand
///
/// This struct defines all the parameters required to append DuckDB
/// extension metadata to an existing dynamic library file.
#[derive(Args, Debug)]
pub struct PackageOptions {
    /// Path to the input dynamic library file
    #[arg(short = 'i', long, value_name = "LIBRARY-PATH")]
    pub library_path: String,

    /// Path where the output extension file should be created
    #[arg(short = 'o', long, value_name = "EXTENSION-PATH")]
    pub extension_path: String,

    /// Version of the extension (e.g., "v1.0.0")
    #[arg(short = 'v', long, value_name = "EXTENSION-VERSION")]
    pub extension_version: String,

    /// Target platform identifier (e.g., "osx_arm64", "linux_amd64")
    #[arg(short = 'p', long, value_name = "DUCKDB-PLATFORM")]
    pub duckdb_platform: String,

    /// Version specification (DuckDB version or C API version)
    #[command(flatten)]
    version: Version,
}

/// Version specification for DuckDB extensions
///
/// This struct represents mutually exclusive version options:
/// either a DuckDB version or a DuckDB C API version.
#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Version {
    /// DuckDB version the extension is built for (e.g., "v1.4.2")
    #[arg(short = 'd', long, value_name = "DUCKDB-VERSION")]
    duckdb_version: Option<String>,

    /// DuckDB C API version the extension is built for (e.g., "v1.2.0")
    #[arg(short = 'a', long, value_name = "DUCKDB-CAPI-VERSION")]
    duckdb_capi_version: Option<String>,
}

impl PackageOptions {
    /// Returns the DuckDB version or DuckDB C API version
    ///
    /// # Panics
    /// Panics if neither version is specified (should be prevented by clap).
    pub fn version(&self) -> String {
        let version = self.version.duckdb_version.as_ref()
            .or(self.version.duckdb_capi_version.as_ref())
            .expect("Missing DuckDB or C API version");
        version.to_string()
    }

    /// Determines the ABI type based on version specification
    ///
    /// Returns `C_STRUCT` for C API versions (stable ABI) or
    /// `C_STRUCT_UNSTABLE` for regular DuckDB versions (unstable ABI).
    pub fn abi_type(&self) -> String {
        let abi_type = if self.version.duckdb_capi_version.is_some() {
            STABLE_ABI_TYPE
        } else {
            UNSTABLE_ABI_TYPE
        };
        abi_type.to_string()
    }
}