# windows-cli-tools

Unix CLI tools for Windows. Native `.exe` files for `cmd.exe` and PowerShell, with no dependencies, no WSL, no MSYS2.

Handles UTF-16 files correctly (PowerShell's default encoding), with automatic BOM detection and transcoding.

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

**x64 (recommended):** `windows-cli-tools-x64.zip`
**32-bit / Windows 7:** `windows-cli-tools-32bit.zip`

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

## Build from Source

Requires [Rust](https://rustup.rs/) 1.75.0 or later.

```
git clone https://github.com/peteretelej/windows-cli-tools.git
cd windows-cli-tools
cargo build --release --workspace
```

Binaries are in `target/release/`.

## License

MIT
