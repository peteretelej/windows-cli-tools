---
title: head
description: Print the first lines of a file
---

# head

Output the first part of files.

## Usage

```
head [OPTIONS] [FILE...]
```

Reads from stdin when no files are specified.

## Options

| Option | Description |
|--------|-------------|
| `-n, --lines <NUM>` | Output first NUM lines (default: 10) |
| `-c, --bytes <NUM>` | Output first NUM bytes |
| `--help` | Display help |
| `--version` | Display version |

The `-N` shorthand is supported (e.g. `head -5` is equivalent to `head -n 5`).

## Examples

```
head file.txt                  # first 10 lines
head -n 20 file.txt            # first 20 lines
head -5 file.txt               # first 5 lines (shorthand)
head -c 100 file.txt           # first 100 bytes
head file1.txt file2.txt       # first 10 lines of each file
type file.txt | head           # first 10 lines from stdin
```

## Notes

- Multiple files display a `==> filename <==` header before each file's output.
- Handles UTF-8, UTF-16 LE, and UTF-16 BE files via BOM detection.
- Line mode counts `\n` and `\r\n` as line terminators.
- Byte mode operates on raw bytes after encoding transcoding.
