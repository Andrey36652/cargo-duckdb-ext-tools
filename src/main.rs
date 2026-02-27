//! Main entry point for the cargo-duckdb-ext command line tool
//!
//! This module handles command line argument parsing and dispatches
//! to the appropriate subcommand implementation.

use cargo_duckdb_ext_tools::commands::Command;
use cargo_duckdb_ext_tools::error::ToolsError;
use cargo_duckdb_ext_tools::options::Options;
use clap::Parser;
use std::env::args;

fn main() -> Result<(), ToolsError> {
    let args = args().into_iter()
        .enumerate()
        .filter(|(index, argument)| *index != 1 || argument != "duckdb-ext")
        .map(|(_, argument)| argument)
        .collect::<Vec<_>>();
    let options: Options = Options::parse_from(args);
    let mut task: Box<dyn Command> = options.try_into()?;
    task.execute()?;
    Ok(())
}
