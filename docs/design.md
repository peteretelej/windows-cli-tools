---
title: Architecture
description: Design overview of windows-cli-tools
---

# Architecture

## Workspace Layout

Cargo workspace with a shared library and 10 binary crates:

```
crates/
  common/     Shared library (encoding, CLI helpers, error formatting)
  head/       Print first N lines/bytes
  tail/       Print last N lines/bytes
  wc/         Count lines, words, bytes, characters
  cat/        Concatenate files to stdout
  tac/        Reverse file line order
  grep/       Search files for regex patterns
  tee/        Copy stdin to stdout and files
  touch/      Create files / update timestamps
  which/      Locate executables on PATH
  yes/        Repeatedly output a string
```

Workspace inheritance in the root `Cargo.toml` keeps each tool's manifest minimal. Version, edition, license, and MSRV are defined once at the workspace level.

## Encoding Strategy

Windows PowerShell 5.1 (still the default shell on Windows 10/11) writes UTF-16 LE with BOM when using `>` redirection. Most existing Unix-on-Windows tools (uutils, BusyBox-w32, GnuWin32) operate on raw bytes and break on these files.

This project handles encoding from day one:

- All I/O in binary mode (no runtime CRLF translation)
- BOM sniffing on file open: UTF-8 BOM, UTF-16 LE BOM, UTF-16 BE BOM
- UTF-16 automatically transcoded to UTF-8
- BOMs stripped from output
- Files without BOM pass through as raw bytes (assumed UTF-8)
- Line splitting recognizes both `\n` and `\r\n`

The encoding layer lives in `crates/common/src/encoding.rs` and wraps `encoding_rs_io::DecodeReaderBytesBuilder`. Every text-processing tool uses `open_input()` or `open_input_or_stdin()` which returns an `impl BufRead` that transparently handles encoding.

Tools that do not process text content (`touch`, `which`, `yes`) do not depend on the `common` crate.

## Dependencies

| Crate | License | Used by | Purpose |
|-------|---------|---------|---------|
| `encoding_rs_io` | MIT/Apache-2.0 | `common` | BOM detection, UTF-16 transcoding |
| `encoding_rs` | MIT/Apache-2.0 | transitive | Encoding engine (Mozilla) |
| `lexopt` | MIT/Apache-2.0 | all tools | Zero-dependency argument parser |
| `regex-lite` | MIT/Apache-2.0 | `grep` | Regex matching, zero transitive deps |
| `tempfile` | MIT/Apache-2.0 | tests | Temporary files in test harness |

All dependencies are MIT/Apache-2.0 dual-licensed.

`lexopt` was chosen over `clap` for minimal binary size and zero transitive dependencies. Each tool parses its own arguments in a small match block.

`regex-lite` was chosen over `regex` to avoid pulling in the full regex engine and its proc-macro dependency tree. It covers the regex subset needed for grep.

## Build Pipeline

### CI (ci.yml)

Runs on every push to main and on pull requests:

- `cargo fmt --check` - formatting
- `cargo clippy --workspace -- -D warnings` - linting
- `cargo test --workspace` - tests
- 32-bit build verification with Rust 1.75.0 targeting `i686-pc-windows-msvc`

### Release (release.yml)

Triggered by version tags (`v*.*.*`). Build matrix:

| Target | Toolchain | Output |
|--------|-----------|--------|
| `x86_64-pc-windows-msvc` | stable | x64 binaries |
| `i686-pc-windows-msvc` | 1.75.0 | 32-bit binaries |

Both targets use static CRT linking (`-C target-feature=+crt-static`) for zero-dependency executables.

Release artifacts: `windows-cli-tools-x64.zip`, `windows-cli-tools-32bit.zip`, and individual x64 `.exe` files.

## MSRV

Minimum Supported Rust Version is **1.75.0**, chosen as the last Rust release with built-in Windows 7 support. The CI verifies the 32-bit build compiles with this toolchain. The x64 build uses latest stable.
