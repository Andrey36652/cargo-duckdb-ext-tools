# cargo-duckdb-ext-tools

[![Crates.io](https://img.shields.io/crates/v/cargo-duckdb-ext-tools.svg)](https://crates.io/crates/cargo-duckdb-ext-tools)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.93%2B-blue.svg)](https://www.rust-lang.org)

一个全面的 Cargo 插件，用于纯 Rust 环境下开发、构建和打包 DuckDB 扩展。消除 Python 依赖，为 Rust 开发者提供无缝的工作流程。

## 🚀 概述

DuckDB 扩展是动态库文件（`.dylib`/`.so`/`.dll`），在文件末尾附加了一个 534 字节的元数据页脚。官方的 DuckDB Rust 扩展模板依赖 Python 脚本来添加此元数据，要求开发者同时维护 Rust 和 Python 环境。

**cargo-duckdb-ext-tools** 通过提供原生 Rust 工具解决了这个问题，与 Cargo 工作流无缝集成，为完整的扩展开发生命周期提供三个基本子命令。

### ✨ 主要特性

- **零非 Rust 依赖**: 纯 Rust 实现，无需 Python 或外部工具
- **完整的开发工作流**: `new` → `build` → `package` 一体化工具
- **智能默认值**: 从 Cargo 元数据和构建工件自动推断参数
- **跨平台支持**: 所有主要平台的原生构建和交叉编译
- **专业日志输出**: 兼容 Cargo 的输出格式，支持颜色
- **ABI 版本支持**: 稳定（`C_STRUCT`）和不稳定（`C_STRUCT_UNSTABLE`）ABI 类型

## 📦 安装

```bash
cargo install cargo-duckdb-ext-tools
```

工具安装为 `cargo-duckdb-ext`，提供三个子命令：`new`、`build` 和 `package`。

## 🚀 完整工作流程示例

以下是从项目创建到在 DuckDB 中运行扩展的完整示例：

```bash
# 1. 创建新的扩展项目
cargo duckdb-ext new quack

# 2. 进入项目目录
cd quack

# 3. 构建和打包扩展
cargo duckdb-ext build -- --release

# 4. 扩展现在已准备就绪：
# target/release/quack.duckdb_extension

# 5. 在 DuckDB 中加载并测试扩展
duckdb -unsigned -c "load 'target/release/quack.duckdb_extension'; from quack('Joe')"
```

**预期输出：**
```
┌───────────────┐
│     🐥       │
│   varchar     │
├───────────────┤
│ Hello Joe     │
└───────────────┘
```

这将创建一个表函数扩展，为任何输入名称返回 "Hello {name}"。

## 🛠️ 子命令

### 1. `cargo duckdb-ext new` - 创建新扩展项目

创建完整的 DuckDB 扩展项目，包含正确的配置和模板代码。

```bash
cargo duckdb-ext new [选项] <路径>
```

#### 基本用法
```bash
# 创建表函数扩展（默认）
cargo duckdb-ext new my-extension

# 创建标量函数扩展
cargo duckdb-ext new --scalar my-scalar-extension
```

#### 关键选项
- `--table` / `--scalar`: 选择函数类型（表函数为默认）
- `--name <名称>`: 设置包名（默认为目录名）
- `--edition <年份>`: Rust 版本（2015、2018、2021、2024）
- `--vcs <版本控制系统>`: 版本控制系统（git、hg、pijul、fossil、none）

#### 创建内容
- 包含 `cdylib` 配置的完整 Cargo 项目
- DuckDB 依赖（`duckdb`、`libduckdb-sys`、`duckdb-ext-macros`）
- 表函数或标量函数的模板代码
- 发布配置文件优化（LTO、strip）

### 2. `cargo duckdb-ext build` - 构建和打包扩展

高级命令，结合编译和打包，具有智能默认值。

```bash
cargo duckdb-ext build [选项] [-- <CARGO_BUILD参数>...]
```

#### 基本用法
```bash
# 使用发布优化构建
cargo duckdb-ext build -- --release

# 从 macOS 交叉编译到 Linux
cargo duckdb-ext build -- --release --target x86_64-unknown-linux-gnu
```

#### 智能默认值
工具自动检测：
- **库路径**: 从构建输出中的 `cdylib` 工件
- **扩展路径**: 库旁边的 `<包名>.duckdb_extension`
- **扩展版本**: 从 `Cargo.toml` 版本字段（前缀为 "v"）
- **平台**: 从目标三元组或主机系统
- **DuckDB 版本**: 从 `duckdb` 或 `libduckdb-sys` 依赖

#### 关键选项
- `-m, --manifest-path`: `Cargo.toml` 路径
- `-o, --extension-path`: 覆盖输出扩展路径
- `-v, --extension-version`: 覆盖扩展版本
- `-p, --duckdb-platform`: 覆盖目标平台
- `-d, --duckdb-version`: 指定 DuckDB 版本
- `-a, --duckdb-capi-version`: 指定 DuckDB C API 版本（启用稳定 ABI）

### 3. `cargo duckdb-ext package` - 打包现有库

低级命令，将 DuckDB 元数据附加到现有的动态库。

```bash
cargo duckdb-ext package --library-path <库路径> --extension-path <输出路径> \
  --extension-version <版本> --duckdb-platform <平台> \
  (--duckdb-version <版本> | --duckdb-capi-version <版本>)
```

#### 示例
```bash
cargo duckdb-ext package \
  -i target/release/libmy_extension.dylib \
  -o my_extension.duckdb_extension \
  -v v1.0.0 \
  -p osx_arm64 \
  -d v1.4.2
```

## 🌍 平台支持

### 支持平台
- **macOS**: Apple Silicon (arm64) 和 Intel (x86_64)
- **Linux**: x86_64、aarch64、x86、arm
- **Windows**: x86_64、x86（通过交叉编译）

### 平台映射
工具自动将 Rust 目标三元组映射到 DuckDB 平台标识符：

| Rust 目标三元组 | DuckDB 平台 |
|----------------|-------------|
| `x86_64-apple-darwin` | `osx_amd64` |
| `aarch64-apple-darwin` | `osx_arm64` |
| `x86_64-unknown-linux-gnu` | `linux_amd64` |
| `aarch64-unknown-linux-gnu` | `linux_arm64` |
| `x86_64-pc-windows-msvc` | `windows_amd64` |
| `i686-pc-windows-msvc` | `windows_amd` |

## 🔧 技术细节

### ABI 类型
- **`C_STRUCT_UNSTABLE`**: 针对 DuckDB 版本构建的扩展的默认值
- **`C_STRUCT`**: 当指定 `--duckdb-capi-version` 时使用（稳定 ABI）

### 元数据结构
534 字节的页脚包括：
1. 起始签名（22 字节）
2. 3 个保留字段（96 字节）
3. ABI 类型（32 字节）
4. 扩展版本（32 字节）
5. DuckDB/C API 版本（32 字节）
6. 平台标识符（32 字节）
7. 魔术版本 "4"（32 字节）
8. 8 个签名填充字段（256 字节）

### 构建集成
`build` 子命令使用 `cargo build --message-format=json` 来：
1. 使用用户提供的参数执行构建
2. 解析 JSON 输出以找到 `cdylib` 工件
3. 从 Cargo.toml 提取包元数据
4. 为缺失参数应用智能默认值
5. 创建并执行打包命令

## 📚 项目结构

```
src/
├── main.rs              # CLI 入口点
├── lib.rs               # 库导出
├── error.rs             # 错误类型
├── logging.rs           # 兼容 Cargo 的日志
├── helpers.rs           # 文件系统工具
├── commands/            # 子命令实现
│   ├── mod.rs          # 命令 trait 和分发
│   ├── new_command.rs  # 项目创建
│   ├── build_command.rs # 构建和打包
│   └── package_command.rs # 元数据附加
└── options/             # CLI 选项解析
    ├── mod.rs          # 顶层选项
    ├── new_option.rs   # new 命令选项
    ├── build_option.rs # build 命令选项
    └── package_option.rs # package 命令选项
```

## 🧪 测试

```bash
# 以开发模式构建工具
cargo build

# 运行测试
cargo test

# 本地安装进行测试
cargo install --path .
```

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建您的功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m '添加一些很棒的功能'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 🆘 支持

- **GitHub Issues**: [https://github.com/redraiment/cargo-duckdb-ext-tools/issues](https://github.com/redraiment/cargo-duckdb-ext-tools/issues)
- **邮箱**: Zhang, Zepeng <redraiment@gmail.com>

## 📄 许可证

本项目基于 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- DuckDB 团队创建了优秀的数据库和扩展系统
- Rust 社区提供了惊人的工具生态系统
- 本项目的所有贡献者和用户

---

**cargo-duckdb-ext-tools** 使 Rust 中的 DuckDB 扩展开发成为一流的体验，消除了外部依赖，提供了从项目创建到打包扩展的无缝工作流程。