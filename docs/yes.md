---
title: yes
description: Repeatedly output a string
---

# yes

Repeatedly output STRING (default: `y`) followed by a newline.

## Usage

```
yes [STRING]
```

## Options

| Option | Description |
|--------|-------------|
| `--help` | Display help |
| `--version` | Display version |

## Examples

```
yes                            # outputs "y" repeatedly
yes n                          # outputs "n" repeatedly
yes hello world                # outputs "hello world" repeatedly
yes | head -5                  # outputs "y" five times
```

## Notes

- Multiple arguments are joined with spaces.
- Uses an 8 KB pre-filled buffer for high-throughput output.
- Handles broken pipe gracefully (exits cleanly when piped to commands like `head`).
- Does not depend on the `common` encoding crate.
