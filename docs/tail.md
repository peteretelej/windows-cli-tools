---
title: tail
description: Print the last lines of a file
---

# tail

Output the last part of files.

## Usage

```
tail [OPTIONS] [FILE...]
```

Reads from stdin when no files are specified.

## Options

| Option | Description |
|--------|-------------|
| `-n, --lines <NUM>` | Output last NUM lines (default: 10) |
| `-c, --bytes <NUM>` | Output last NUM bytes |
| `--help` | Display help |
| `--version` | Display version |

The `-N` shorthand is supported (e.g. `tail -5` is equivalent to `tail -n 5`).

## Examples

```
tail file.txt                  # last 10 lines
tail -n 20 file.txt            # last 20 lines
tail -5 file.txt               # last 5 lines (shorthand)
tail -c 100 file.txt           # last 100 bytes
tail file1.txt file2.txt       # last 10 lines of each file
type file.txt | tail           # last 10 lines from stdin
```

## Notes

- Multiple files display a `==> filename <==` header before each file's output.
- Line mode uses a ring buffer for memory-efficient operation.
- Handles UTF-8, UTF-16 LE, and UTF-16 BE files via BOM detection.
