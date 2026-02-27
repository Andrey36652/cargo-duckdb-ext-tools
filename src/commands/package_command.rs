//! Implementation of the `package` subcommand
//!
//! This module handles the low-level packaging of dynamic libraries
//! into DuckDB extensions by appending the 534-byte metadata footer.

use crate::commands::Command;
use crate::error::ToolsError;
use crate::finished;
use crate::helpers::open_duplicate;
use crate::logging;
use crate::options::package_option::PackageOptions;
use crate::packing;
use std::fs::File;
use std::io::Write;

/// ABI type for extensions built against stable DuckDB C API
pub(crate) const STABLE_ABI_TYPE: &'static str = "C_STRUCT";
/// ABI type for extensions built against unstable DuckDB C API
pub(crate) const UNSTABLE_ABI_TYPE: &'static str = "C_STRUCT_UNSTABLE";

/// Command for packaging dynamic libraries as DuckDB extensions
///
/// This struct holds all the metadata needed to create a DuckDB
/// extension file and provides methods to write the 534-byte footer.
pub(crate) struct PackageCommand {
    /// File handle opened in append mode for writing metadata
    pub(crate) file: File,
    /// Path where the output extension file should be created
    pub(crate) extension_path: String,
    /// Version string of the extension (must start with 'v')
    pub(crate) extension_version: String,
    /// Target platform identifier (e.g., "osx_arm64", "linux_amd64")
    pub(crate) duckdb_platform: String,
    /// DuckDB (C API) version compatibility
    pub(crate) duckdb_version: String,
    /// ABI type specification (C_STRUCT or C_STRUCT_UNSTABLE)
    pub(crate) abi_type: String,
}

impl TryFrom<PackageOptions> for PackageCommand {
    type Error = ToolsError;

    /// Constructs a PackageCommand from command line options
    ///
    /// This conversion sets up the global quiet flag and creates the
    /// extension file by duplicating the source library.
    fn try_from(options: PackageOptions) -> Result<Self, Self::Error> {
        let file = open_duplicate(&options.library_path, &options.extension_path)?;
        let duckdb_version = options.version();
        let abi_type = options.abi_type();
        Ok(Self {
            file,
            extension_path: options.extension_path,
            extension_version: options.extension_version,
            duckdb_platform: options.duckdb_platform,
            duckdb_version,
            abi_type,
        })
    }
}

impl Command for PackageCommand {
    /// Executes the packaging operation
    ///
    /// This method writes the complete 534-byte metadata footer
    /// to the extension file.
    fn execute(&mut self) -> Result<(), ToolsError> {
        self.write_metadata()
    }
}

impl PackageCommand {
    /// Appends the 534-byte DuckDB extension metadata footer to the file
    ///
    /// This method writes the complete metadata structure including:
    /// - Start signature
    /// - Reserved fields
    /// - ABI type
    /// - Extension version
    /// - DuckDB version
    /// - Platform identifier
    /// - Unknown field (always "4")
    /// - Signature padding
    fn write_metadata(&mut self) -> Result<(), ToolsError> {
        Self::write_start_signature(&mut self.file)?;
        // Write 3 empty 32-byte fields (reserved for future use)
        for _ in 0..3 {
            Self::write_field(&mut self.file, "")?;
        }
        packing!("ABI Type ({})", self.abi_type);
        Self::write_field(&mut self.file, self.abi_type.as_str())?;
        packing!("Extension Version ({})", self.extension_version);
        Self::write_field(&mut self.file, self.extension_version.as_str())?;
        let version_kind = if self.abi_type == STABLE_ABI_TYPE {
            "DuckDB C API Version"
        } else {
            "DuckDB Version"
        };
        packing!("{} ({})", version_kind, self.duckdb_version);
        Self::write_field(&mut self.file, self.duckdb_version.as_str())?;
        packing!("DuckDB Platform ({})", self.duckdb_platform);
        Self::write_field(&mut self.file, self.duckdb_platform.as_str())?;
        // Magic version, always "4" in current DuckDB format
        Self::write_field(&mut self.file, "4")?;
        // Write 8 empty 32-byte fields for signature padding
        for _ in 0..8 {
            Self::write_field(&mut self.file, "")?;
        }
        finished!("DuckDB Extension ({})", self.extension_path);
        Ok(())
    }

    /// Writes the fixed start signature for DuckDB extension files
    ///
    /// The signature is a specific byte sequence that identifies the file
    /// as a DuckDB extension. This is the first part of the 534-byte footer.
    fn write_start_signature(file: &mut File) -> Result<(), std::io::Error> {
        file.write_all(&vec![
            0, 147, 4, 16, 100, 117, 99, 107, 100, 98, 95, 115, 105, 103, 110, 97, 116, 117, 114,
            101, 128, 4,
        ])
    }

    /// Writes a 32-byte field with the given content
    ///
    /// This pads the content to exactly 32 bytes with null bytes.
    /// If the content is longer than 32 bytes, it will be truncated.
    fn write_field(file: &mut File, content: &str) -> Result<(), std::io::Error> {
        let mut bytes = [0u8; 32];
        bytes[..content.len()].copy_from_slice(content.as_bytes());
        file.write_all(bytes.as_ref())
    }
}