---
title: cat
description: Concatenate files and print to stdout
---

# cat

Concatenate files and print on the standard output.

## Usage

```
cat [OPTIONS] [FILE...]
```

Reads from stdin when no files are specified.

## Options

| Option | Description |
|--------|-------------|
| `-n, --number` | Number all output lines |
| `--raw` | Copy files byte-for-byte without encoding handling |
| `--help` | Display help |
| `--version` | Display version |

## Examples

```
cat file.txt                   # print file contents
cat file1.txt file2.txt        # concatenate two files
cat -n file.txt                # print with line numbers
cat --raw file.txt             # byte-for-byte copy (no BOM handling)
type input.txt | cat -n        # number lines from stdin
```

## Notes

- Line numbering is continuous across multiple files.
- Default mode transcodes UTF-16 files to UTF-8 and strips BOMs.
- `--raw` mode bypasses all encoding handling, copying raw bytes directly. Useful for binary files or when encoding should be preserved as-is.
- Handles UTF-8, UTF-16 LE, and UTF-16 BE files via BOM detection.
