---
title: grep
description: Search files for patterns
---

# grep

Search for PATTERN in each FILE (or stdin).

## Usage

```
grep [OPTIONS] PATTERN [FILE...]
```

Reads from stdin when no files are specified.

## Options

| Option | Description |
|--------|-------------|
| `-i, --ignore-case` | Case-insensitive matching |
| `-n, --line-number` | Prefix each match with its line number |
| `-v, --invert-match` | Select non-matching lines |
| `-c, --count` | Print only a count of matching lines per file |
| `-l, --files-with-matches` | Print only names of files with matches |
| `-r, --recursive` | Search directories recursively |
| `--help` | Display help |
| `--version` | Display version |

## Examples

```
grep error log.txt                 # search for "error" in file
grep -i warning log.txt            # case-insensitive search
grep -n TODO src/*.rs              # show matches with line numbers
grep -v "^#" config.txt            # exclude comment lines
grep -c error *.log                # count matches per file
grep -l TODO src/*.rs              # list files containing matches
grep -r "fn main" src/             # recursive search in directory
type log.txt | grep error          # search stdin
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Matches found |
| 1 | No matches found |
| 2 | Error occurred |

## Notes

- Patterns are regular expressions (powered by `regex-lite`).
- Recursive mode (`-r`) skips hidden files (names starting with `.`) and binary files.
- Binary file detection examines the first 8 KB for null bytes.
- When searching multiple files, output lines are prefixed with the filename.
- Handles UTF-8, UTF-16 LE, and UTF-16 BE files via BOM detection.
