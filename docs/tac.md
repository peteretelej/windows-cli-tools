---
title: tac
description: Print files in reverse line order
---

# tac

Concatenate and print files in reverse.

## Usage

```
tac [FILE...]
```

Reads from stdin when no files are specified.

## Options

| Option | Description |
|--------|-------------|
| `--help` | Display help |
| `--version` | Display version |

## Examples

```
tac file.txt                   # print file lines in reverse order
tac file1.txt file2.txt        # reverse each file
type file.txt | tac            # reverse stdin
```

## Notes

- Each file is reversed independently (file2 is not appended to file1 before reversing).
- Reads the entire file into memory to reverse lines.
- Handles UTF-8, UTF-16 LE, and UTF-16 BE files via BOM detection.
