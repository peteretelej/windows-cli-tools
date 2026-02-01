---
title: touch
description: Create files or update timestamps
---

# touch

Create FILE if it does not exist, or update its modification time.

## Usage

```
touch [FILE...]
```

## Options

| Option | Description |
|--------|-------------|
| `--help` | Display help |
| `--version` | Display version |

## Examples

```
touch newfile.txt                 # create empty file
touch existing.txt                # update modification time
touch file1.txt file2.txt         # touch multiple files
```

## Notes

- If the file does not exist, an empty file is created.
- If the file exists, its modification time is updated to the current time.
- Errors are reported per-file; the exit code is 1 if any file fails.
- Does not depend on the `common` encoding crate.
