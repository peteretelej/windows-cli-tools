# windows-cli-tools

Unix CLI tools (head, tail, cat, grep, wc and more) as native Windows .exe files - zero runtime dependencies.

[![CI](https://img.shields.io/github/actions/workflow/status/peteretelej/windows-cli-tools/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/peteretelej/windows-cli-tools/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/peteretelej/windows-cli-tools?style=flat-square)](https://github.com/peteretelej/windows-cli-tools/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)

Standalone `.exe` files that work in cmd, PowerShell, and any Windows terminal. Download, add to your PATH, and use them like you would on Linux or macOS. No installer, no WSL, no MSYS2.

Supports Windows 7 and later. All tools read from stdin and support piping, just like their Unix counterparts. UTF-16 files are handled correctly (PowerShell's default encoding), with automatic BOM detection and transcoding.

## Tools

| Tool | Description |
|------|-------------|
| [head](docs/head.md) | Print the first lines/bytes of a file |
| [tail](docs/tail.md) | Print the last lines/bytes of a file |
| [wc](docs/wc.md) | Count lines, words, bytes, characters |
| [cat](docs/cat.md) | Concatenate files to stdout |
| [tac](docs/tac.md) | Print files in reverse line order |
| [grep](docs/grep.md) | Search files for regex patterns |
| [tee](docs/tee.md) | Copy stdin to stdout and files |
| [touch](docs/touch.md) | Create files or update timestamps |
| [which](docs/which.md) | Locate executables on PATH |
| [yes](docs/yes.md) | Repeatedly output a string |

Looking for `tree`? See [peteretelej/tree](https://github.com/peteretelej/tree).

## Install

### From GitHub Releases

Download the latest zip or individual `.exe` files from [Releases](https://github.com/peteretelej/windows-cli-tools/releases). Place the executables in a directory on your PATH.

- **x64 (recommended):** `windows-cli-tools-x64.zip`
- **32-bit (has Win7 Support):** `windows-cli-tools-32bit.zip`

### From source

```
cargo install --git https://github.com/peteretelej/windows-cli-tools head tail wc cat tac grep tee touch which yes
```

## Usage

```
head -n 20 file.txt            # first 20 lines
tail -5 file.txt               # last 5 lines
wc -l *.txt                    # count lines in all txt files
cat -n file.txt                # print with line numbers
grep -rn "TODO" src/           # recursive search with line numbers
tee output.txt                 # copy stdin to file and stdout
touch newfile.txt              # create file
which python                   # find executable on PATH
yes | head -3                  # output "y" three times
```

## Use with AI Agents

These tools are useful for AI agents and coding assistants running on Windows. Most AI tools expect standard CLI commands like `head`, `grep`, and `cat` to be available. After installing and confirming the tools are accessible from your command prompt, add a line to your agent's system prompt or instructions:

```
The following CLI tools are available on this system: head, tail, cat, tac, grep, wc, tee, touch, which, yes. Use them for file and text operations.
```

> **Tip:** Git Bash for Windows also provides these commands, but agents often have trouble with it when switching between terminals or running inline commands. Native `.exe` files on PATH work reliably across cmd, PowerShell, and any terminal an agent might use.

For a more complete toolkit, consider also installing [ripgrep](https://github.com/BurntSushi/ripgrep), [fd](https://github.com/sharkdp/fd), [jq](https://github.com/jqlang/jq), and [tree](https://github.com/peteretelej/tree) - all have prebuilt Windows binaries.

## Build from Source

Requires [Rust](https://rustup.rs/) 1.75.0 or later.

```
git clone https://github.com/peteretelej/windows-cli-tools.git

cd windows-cli-tools

cargo build --release --workspace
```

Binaries are in `target/release/`.

For Windows 7 or 32-bit builds, use Rust 1.75.0 targeting i686 with static CRT linking. This does not change your default Rust toolchain. Run in PowerShell:

```powershell
rustup toolchain install 1.75.0 --target i686-pc-windows-msvc

$env:RUSTFLAGS = "-C target-feature=+crt-static"

cargo +1.75.0 build --release --workspace --target i686-pc-windows-msvc
```

Binaries are in `target/i686-pc-windows-msvc/release/`.

## Contributing

Contributions are welcome! 

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines on building, testing, and submitting changes.

## License

MIT
