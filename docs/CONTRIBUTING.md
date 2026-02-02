---
title: Contributing
description: Guide for contributing to windows-cli-tools
---

# Contributing

## Prerequisites

- **Rust 1.75.0+** (install via [rustup](https://rustup.rs/))
- **Windows** for full testing (CI runs on `windows-latest`)
- `rustfmt` and `clippy` components: `rustup component add rustfmt clippy`

## Build and Test

```
cargo build --workspace              # build all tools
cargo test --workspace               # run all tests
cargo fmt --check                    # check formatting
cargo clippy --workspace -- -D warnings  # lint
```

Build a single tool:

```
cargo build -p head
```

Run a single tool's tests:

```
cargo test -p head
```

## Adding a New Tool

1. Create `crates/toolname/Cargo.toml`:

```toml
[package]
name = "toolname"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
common = { workspace = true }  # omit if the tool doesn't process text
lexopt = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
```

2. Create `crates/toolname/src/main.rs` with argument parsing via `lexopt`, `--help`/`--version` support, and the tool logic.

3. Add `docs/toolname.md` following the existing tool doc format (frontmatter, usage, options table, examples, notes).

4. The workspace auto-discovers crates via `members = ["crates/*"]`, so no root manifest changes are needed.

## Git Hooks

A pre-push hook is provided in `scripts/pre-push` that runs the same checks as CI (fmt, clippy, test) before allowing a push. To install it:

```
# Unix / Git Bash
ln -sf ../../scripts/pre-push .git/hooks/pre-push

# PowerShell (run as admin, or with Developer Mode enabled)
New-Item -ItemType SymbolicLink -Path .git\hooks\pre-push -Target ..\..\scripts\pre-push -Force
```

## Code Style

- `cargo fmt` for formatting (enforced in CI)
- `cargo clippy -- -D warnings` for linting (enforced in CI)
- No unnecessary comments; code should be self-documenting
- Use `common::error::err()` for fatal errors and `common::error::warn()` for non-fatal warnings
- Use `common::encoding::open_input()` or `open_input_or_stdin()` for text file reading
- Handle broken pipe errors gracefully in I/O loops

## Pull Requests

- One tool or feature per PR
- Include tests for new functionality
- Ensure `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, and `cargo test --workspace` pass
- Keep commits focused and descriptive
