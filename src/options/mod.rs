//! Command line option parsing for cargo-duckdb-ext
//!
//! This module defines the command line interface using the `clap` crate,
//! providing structured access to all subcommands and their options.

pub mod build_option;
pub mod new_option;
pub mod package_option;

use crate::options::build_option::BuildOptions;
use crate::options::new_option::NewOptions;
use crate::options::package_option::PackageOptions;
use clap::Parser;
use clap::Subcommand;

/// Top-level command line options for cargo-duckdb-ext
///
/// This struct captures global options and dispatches to subcommands
/// via the `Commands` enum.
#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct Options {
    /// Suppress console output
    #[arg(short = 'q', long, default_value_t = false)]
    pub quiet: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for cargo-duckdb-ext
///
/// Each variant corresponds to a specific functionality:
/// - `Build`: Compile and package extensions in one step
/// - `Package`: Append metadata to existing dynamic libraries
/// - `New`: Create new DuckDB extension projects
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build and package DuckDB extensions
    Build(BuildOptions),
    /// Package existing dynamic libraries as DuckDB extensions
    Package(PackageOptions),
    /// Create new DuckDB extension projects
    New(NewOptions),
}
