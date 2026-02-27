//! Command line options for the `new` subcommand
//!
//! This module defines parameters for creating new DuckDB extension projects,
//! including project configuration, version control, and function type selection.

use clap::Args;
use clap::ValueEnum;
use std::fmt::Display;

/// Command line options for the `new` subcommand
///
/// This struct defines parameters for creating new DuckDB extension projects,
/// mirroring many of the options from `cargo new` with additional
/// DuckDB-specific functionality.
#[derive(Args, Debug)]
pub struct NewOptions {
    /// Generate a table function (default)
    #[arg(long, default_value_t = true, conflicts_with = "scalar")]
    pub table: bool,

    /// Generate a scalar function instead of a table function
    #[arg(long, default_value_t = false, conflicts_with = "table")]
    pub scalar: bool,

    /// Initialize a new repository for the given version control system
    #[arg(value_enum, long)]
    pub vcs: Option<VersionControlSystem>,

    /// Rust edition to use for the generated crate
    #[arg(value_enum, long, value_name = "YEAR")]
    pub edition: Option<Edition>,

    /// Set the resulting package name (defaults to directory name)
    #[arg(long)]
    pub name: Option<String>,

    /// Registry to use for dependency resolution
    #[arg(long)]
    pub registry: Option<String>,

    /// Override configuration values
    #[arg(long = "config", value_name = "KEY=VALUE|PATH")]
    pub configuations: Vec<String>,

    /// Path where the new project should be created
    pub path: String,
}

/// Supported version control systems for new projects
#[derive(ValueEnum, Clone, Debug)]
pub enum VersionControlSystem {
    /// Git version control system
    Git,
    /// Mercurial version control system
    Hg,
    /// Pijul version control system
    Pijul,
    /// Fossil version control system
    Fossil,
    /// No version control system
    None,
}

impl Display for VersionControlSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            VersionControlSystem::Git => "git",
            VersionControlSystem::Hg => "hg",
            VersionControlSystem::Pijul => "pijul",
            VersionControlSystem::Fossil => "fossil",
            VersionControlSystem::None => "none",
        })
    }
}

/// Rust editions supported for new projects
#[derive(ValueEnum, Clone, Debug)]
pub enum Edition {
    /// Rust 2015 edition
    #[value(name = "2015")]
    E2015,

    /// Rust 2018 edition
    #[value(name = "2018")]
    E2018,

    /// Rust 2021 edition
    #[value(name = "2021")]
    E2021,

    /// Rust 2024 edition
    #[value(name = "2024")]
    E2024,
}

impl Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Edition::E2015 => "2015",
            Edition::E2018 => "2018",
            Edition::E2021 => "2021",
            Edition::E2024 => "2024",
        })
    }
}
