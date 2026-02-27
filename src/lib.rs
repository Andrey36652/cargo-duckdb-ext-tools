//! Cargo plugin for DuckDB extension development
//!
//! This crate provides a Cargo plugin (`cargo-duckdb-ext`) that enables
//! Rust developers to create, build, and package DuckDB extensions
//! without requiring Python or other external tooling.

pub mod commands;
pub mod error;
pub(crate) mod helpers;
pub(crate) mod logging;
pub mod options;
