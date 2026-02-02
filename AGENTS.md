# AGENTS.md

10 Unix CLI tools for Windows, built as a Rust workspace. Drop-in `.exe` files for `cmd.exe` and PowerShell with no external dependencies.

## Workspace

Root: `Cargo.toml` (workspace with `members = ["crates/*"]`, workspace-level version/edition/MSRV/license). MSRV: 1.75.0. License: MIT.

Shared library: `crates/common/` - encoding (BOM sniffing, UTF-16 transcoding via `encoding_rs_io`), CLI helpers (`--help`/`--version`), error formatting (`err()`/`warn()`).

## Tools

| Tool | Crate | Depends on common | Key flags |
|------|-------|-------------------|-----------|
| head | `crates/head/` | yes | `-n NUM`, `-c NUM`, `-N` shorthand |
| tail | `crates/tail/` | yes | `-n NUM`, `-c NUM`, `-N` shorthand |
| wc | `crates/wc/` | yes | `-l`, `-w`, `-c`, `-m` |
| cat | `crates/cat/` | yes | `-n`, `--raw` |
| tac | `crates/tac/` | yes | (none) |
| grep | `crates/grep/` | yes | `-i`, `-n`, `-v`, `-c`, `-l`, `-r` |
| tee | `crates/tee/` | no | `-a` |
| touch | `crates/touch/` | no | (none) |
| which | `crates/which/` | no | `-a` |
| yes | `crates/yes/` | no | (none) |

All tools support `--help` and `--version`. Tools without `common` dependency handle raw bytes only.

## Build and Test

```
cargo build --workspace
cargo test --workspace
cargo fmt --check
cargo clippy --workspace -- -D warnings
```

## Patterns

Argument parsing: `lexopt` with match on `Short`/`Long`/`Value` variants. Help/version: `common::cli::print_help(TOOL, USAGE)` / `print_version(TOOL, VERSION)`. Fatal errors: `common::error::err(TOOL, msg)`. File input: `common::encoding::open_input(path)` or `open_input_or_stdin(opt)` returns `impl BufRead`. Broken pipe: caught and silently ignored in I/O loops. Grep uses `regex-lite` for pattern matching.

## Testing

Unit tests: inline `#[cfg(test)]` modules in each tool's `main.rs`, testing core logic via `Cursor`-based readers.

Integration tests: `crates/tests-integration/` exercises compiled binaries via `std::process::Command`. Shared helpers in `tests/helpers.rs`; fixture files in `tests/fixtures/`.

**Use table-driven tests** to reduce sprawl. Group related scenarios into a single `#[test]` function using `Case`/`StdinCase` structs from helpers, or inline `&[(&str, ...)]` slices iterated in a loop. Each tuple/struct carries a name for identification on failure. Reserve separate `#[test]` functions only for tests needing unique setup (tempdir, custom env, pipe chains).

## CI/CD

CI (`ci.yml`): runs on `windows-latest` - fmt, clippy, test, plus 32-bit build check with Rust 1.75.0.
Release (`release.yml`): triggered by `v*.*.*` tags, builds x64 (stable) and x86 (1.75.0) with static CRT, publishes zips and individual exe files to GitHub Releases.

## Docs

Tool docs in `docs/TOOL.md` (with YAML frontmatter for future Starlight site). Architecture: `docs/design.md`. Contributing: `docs/CONTRIBUTING.md`.
