---
title: which
description: Locate executables on PATH
---

# which

Locate an executable on PATH.

## Usage

```
which [OPTIONS] COMMAND
```

## Options

| Option | Description |
|--------|-------------|
| `-a, --all` | Show all matches, not just the first |
| `--help` | Display help |
| `--version` | Display version |

## Examples

```
which python                   # find python executable
which -a python                # show all python executables on PATH
which git                      # locate git
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Command found |
| 1 | Command not found |

## Notes

- Searches the current directory first, then directories in PATH.
- On Windows, appends extensions from the PATHEXT environment variable (defaults to `.COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC`).
- Extension matching is case-insensitive on Windows.
- Results are deduplicated; duplicate paths are not shown.
- Does not depend on the `common` encoding crate.
