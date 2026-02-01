---
title: wc
description: Print newline, word, and byte counts
---

# wc

Print newline, word, and byte counts for each file.

## Usage

```
wc [OPTIONS] [FILE...]
```

Reads from stdin when no files are specified.

## Options

| Option | Description |
|--------|-------------|
| `-l` | Print line count |
| `-w` | Print word count |
| `-c` | Print byte count |
| `-m` | Print character count (UTF-8 aware) |
| `--help` | Display help |
| `--version` | Display version |

With no flags, prints lines, words, and bytes (in that order).

## Examples

```
wc file.txt                    # lines, words, bytes
wc -l file.txt                 # line count only
wc -w file.txt                 # word count only
wc -c file.txt                 # byte count only
wc -m file.txt                 # character count (UTF-8)
wc file1.txt file2.txt         # counts per file + total
type file.txt | wc -l          # count lines from stdin
```

## Notes

- Multiple files show per-file counts followed by a total row.
- Output columns are right-aligned with dynamic width.
- Words are delimited by whitespace (space, tab, newline, carriage return).
- `-m` counts UTF-8 characters; invalid sequences are counted per-byte as fallback.
- Handles UTF-8, UTF-16 LE, and UTF-16 BE files via BOM detection.
