---
title: tee
description: Copy stdin to stdout and files
---

# tee

Copy stdin to stdout and each FILE.

## Usage

```
tee [OPTIONS] [FILE...]
```

## Options

| Option | Description |
|--------|-------------|
| `-a, --append` | Append to files instead of overwriting |
| `--help` | Display help |
| `--version` | Display version |

## Examples

```
dir | tee output.txt                      # save dir listing to file
dir | tee output.txt | findstr "src"      # save and filter
dir | tee -a log.txt                      # append to existing file
dir | tee file1.txt file2.txt             # write to multiple files
```

## Notes

- Default mode overwrites existing files.
- Uses 8 KB buffer for efficient streaming.
- Flushes output after each write to keep stdout and files in sync.
- Does not perform encoding transcoding; bytes pass through as-is.
