# cargo-duckdb-ext-tools

[![Crates.io](https://img.shields.io/crates/v/cargo-duckdb-ext-tools.svg)](https://crates.io/crates/cargo-duckdb-ext-tools)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.93%2B-blue.svg)](https://www.rust-lang.org)

A comprehensive Cargo plugin for developing, building, and packaging DuckDB extensions in pure Rust. Eliminates Python dependencies and provides a seamless workflow for Rust developers.

## 🚀 Overview

DuckDB extensions are dynamic libraries (`.dylib`/`.so`/`.dll`) with a 534-byte metadata footer appended to the file. The official DuckDB Rust extension template relies on Python scripts for metadata appending, requiring developers to maintain both Rust and Python environments.

**cargo-duckdb-ext-tools** solves this by providing native Rust tooling that integrates seamlessly with Cargo workflows, offering three essential subcommands for the complete extension development lifecycle.

### ✨ Key Features

- **Zero Non-Rust Dependencies**: Pure Rust implementation, no Python or external tools required
- **Complete Development Workflow**: `new` → `build` → `package` in one cohesive tool
- **Intelligent Defaults**: Automatic parameter inference from Cargo metadata and build artifacts
- **Cross-Platform Support**: Native builds and cross-compilation for all major platforms
- **Professional Logging**: Cargo-compatible output format with color support
- **ABI Version Support**: Both stable (`C_STRUCT`) and unstable (`C_STRUCT_UNSTABLE`) ABI types

## 📦 Installation

```bash
cargo install cargo-duckdb-ext-tools
```

The tool installs as `cargo-duckdb-ext` and provides three subcommands: `new`, `build`, and `package`.

## 🚀 Complete Workflow Example

Here's a complete example from project creation to running the extension in DuckDB:

```bash
# 1. Create a new extension project
cargo duckdb-ext new quack

# 2. Enter the project directory
cd quack

# 3. Build and package the extension
cargo duckdb-ext build -- --release

# 4. The extension is now ready at:
# target/release/quack.duckdb_extension

# 5. Load and test the extension in DuckDB
duckdb -unsigned -c "load 'target/release/quack.duckdb_extension'; from quack('Joe')"
```

**Expected output:**
```
┌───────────────┐
│     🐥       │
│   varchar     │
├───────────────┤
│ Hello Joe     │
└───────────────┘
```

This creates a table function extension that returns "Hello {name}" for any input name.

## 🛠️ Subcommands

### 1. `cargo duckdb-ext new` - Create New Extension Projects

Creates a complete DuckDB extension project with proper configuration and template code.

```bash
cargo duckdb-ext new [OPTIONS] <PATH>
```

#### Basic Usage
```bash
# Create a table function extension (default)
cargo duckdb-ext new my-extension

# Create a scalar function extension
cargo duckdb-ext new --scalar my-scalar-extension
```

#### Key Options
- `--table` / `--scalar`: Choose function type (table function is default)
- `--name <NAME>`: Set package name (defaults to directory name)
- `--edition <YEAR>`: Rust edition (2015, 2018, 2021, 2024)
- `--vcs <VCS>`: Version control system (git, hg, pijul, fossil, none)

#### What It Creates
- Complete Cargo project with `cdylib` configuration
- DuckDB dependencies (`duckdb`, `libduckdb-sys`, `duckdb-ext-macros`)
- Template code for table or scalar functions
- Release profile optimizations (LTO, strip)

### 2. `cargo duckdb-ext build` - Build and Package Extensions

The high-level command that combines compilation and packaging with intelligent defaults.

```bash
cargo duckdb-ext build [OPTIONS] [-- <CARGO_BUILD_ARGS>...]
```

#### Basic Usage
```bash
# Build with release optimizations
cargo duckdb-ext build -- --release

# Cross-compile for Linux from macOS
cargo duckdb-ext build -- --release --target x86_64-unknown-linux-gnu
```

#### Intelligent Defaults
The tool automatically detects:
- **Library path**: From `cdylib` artifacts in build output
- **Extension path**: `<package-name>.duckdb_extension` next to the library
- **Extension version**: From `Cargo.toml` version field (prefixed with "v")
- **Platform**: From target triple or host system
- **DuckDB version**: From `duckdb` or `libduckdb-sys` dependency

#### Key Options
- `-m, --manifest-path`: Path to `Cargo.toml`
- `-o, --extension-path`: Override output extension path
- `-v, --extension-version`: Override extension version
- `-p, --duckdb-platform`: Override target platform
- `-d, --duckdb-version`: Specify DuckDB version
- `-a, --duckdb-capi-version`: Specify DuckDB C API version (enables stable ABI)

### 3. `cargo duckdb-ext package` - Package Existing Libraries

Low-level command to append DuckDB metadata to an existing dynamic library.

```bash
cargo duckdb-ext package --library-path <LIB> --extension-path <OUTPUT> \
  --extension-version <VERSION> --duckdb-platform <PLATFORM> \
  (--duckdb-version <VERSION> | --duckdb-capi-version <VERSION>)
```

#### Example
```bash
cargo duckdb-ext package \
  -i target/release/libmy_extension.dylib \
  -o my_extension.duckdb_extension \
  -v v1.0.0 \
  -p osx_arm64 \
  -d v1.4.2
```

## 🌍 Platform Support

### Supported Platforms
- **macOS**: Apple Silicon (arm64) and Intel (x86_64)
- **Linux**: x86_64, aarch64, x86, arm
- **Windows**: x86_64, x86 (via cross-compilation)

### Platform Mapping
The tool automatically maps Rust target triples to DuckDB platform identifiers:

| Rust Target Triple | DuckDB Platform |
|-------------------|-----------------|
| `x86_64-apple-darwin` | `osx_amd64` |
| `aarch64-apple-darwin` | `osx_arm64` |
| `x86_64-unknown-linux-gnu` | `linux_amd64` |
| `aarch64-unknown-linux-gnu` | `linux_arm64` |
| `x86_64-pc-windows-msvc` | `windows_amd64` |
| `i686-pc-windows-msvc` | `windows_amd` |

## 🔧 Technical Details

### ABI Types
- **`C_STRUCT_UNSTABLE`**: Default for extensions built against DuckDB version
- **`C_STRUCT`**: Used when `--duckdb-capi-version` is specified (stable ABI)

### Metadata Structure
The 534-byte footer includes:
1. Start signature (22 bytes)
2. 3 reserved fields (96 bytes)
3. ABI type (32 bytes)
4. Extension version (32 bytes)
5. DuckDB/C API version (32 bytes)
6. Platform identifier (32 bytes)
7. Magic version "4" (32 bytes)
8. 8 signature padding fields (256 bytes)

### Build Integration
The `build` subcommand uses `cargo build --message-format=json` to:
1. Execute the build with user-provided arguments
2. Parse JSON output to find `cdylib` artifacts
3. Extract package metadata from Cargo.toml
4. Apply intelligent defaults for missing parameters
5. Create and execute packaging commands

## 📚 Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library exports
├── error.rs             # Error types
├── logging.rs           # Cargo-compatible logging
├── helpers.rs           # File system utilities
├── commands/            # Subcommand implementations
│   ├── mod.rs          # Command trait and dispatch
│   ├── new_command.rs  # Project creation
│   ├── build_command.rs # Build and package
│   └── package_command.rs # Metadata appending
└── options/             # CLI option parsing
    ├── mod.rs          # Top-level options
    ├── new_option.rs   # New command options
    ├── build_option.rs # Build command options
    └── package_option.rs # Package command options
```

## 🧪 Testing

```bash
# Build the tool in development mode
cargo build

# Run tests
cargo test

# Install locally for testing
cargo install --path .
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🆘 Support

- **GitHub Issues**: [https://github.com/redraiment/cargo-duckdb-ext-tools/issues](https://github.com/redraiment/cargo-duckdb-ext-tools/issues)
- **Email**: Zhang, Zepeng <redraiment@gmail.com>

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The DuckDB team for creating an excellent database and extension system
- The Rust community for the amazing tooling ecosystem
- All contributors and users of this project

---

**cargo-duckdb-ext-tools** makes DuckDB extension development in Rust a first-class experience, eliminating external dependencies and providing a seamless workflow from project creation to packaged extension.