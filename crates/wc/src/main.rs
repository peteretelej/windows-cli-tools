use common::cli;
use common::encoding;
use common::error;
use common::lexopt;
use common::lexopt::prelude::*;
use std::io::{self, Read, Write};

const TOOL: &str = "wc";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
struct Flags {
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

impl Flags {
    fn any_set(&self) -> bool {
        self.lines || self.words || self.bytes || self.chars
    }

    fn default_mode(&self) -> Self {
        if self.any_set() {
            Flags {
                lines: self.lines,
                words: self.words,
                bytes: self.bytes,
                chars: self.chars,
            }
        } else {
            Flags {
                lines: true,
                words: true,
                bytes: true,
                chars: false,
            }
        }
    }
}

#[derive(Default)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

fn count(mut reader: impl Read) -> io::Result<Counts> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let mut c = Counts {
        bytes: buf.len(),
        ..Default::default()
    };

    let mut in_word = false;
    for &b in &buf {
        if b == b'\n' {
            c.lines += 1;
        }
        let is_ws = b == b' ' || b == b'\t' || b == b'\n' || b == b'\r';
        if !is_ws && !in_word {
            c.words += 1;
        }
        in_word = !is_ws;
    }

    let text = String::from_utf8_lossy(&buf);
    c.chars = text.chars().count();

    Ok(c)
}

fn format_counts(counts: &Counts, flags: &Flags, width: usize) -> String {
    let mut parts = Vec::new();
    if flags.lines {
        parts.push(format!("{:>width$}", counts.lines));
    }
    if flags.words {
        parts.push(format!("{:>width$}", counts.words));
    }
    if flags.bytes {
        parts.push(format!("{:>width$}", counts.bytes));
    }
    if flags.chars {
        parts.push(format!("{:>width$}", counts.chars));
    }
    parts.join(" ")
}

fn parse_args() -> (Flags, Vec<String>) {
    let mut flags = Flags::default();
    let mut files = Vec::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser
        .next()
        .unwrap_or_else(|e| error::err(TOOL, &e.to_string()))
    {
        match arg {
            Short('l') => flags.lines = true,
            Short('w') => flags.words = true,
            Short('c') => flags.bytes = true,
            Short('m') => flags.chars = true,
            Long("help") => {
                cli::print_help(TOOL, "print newline, word, and byte counts for each file");
                std::process::exit(0);
            }
            Long("version") => {
                cli::print_version(TOOL, VERSION);
                std::process::exit(0);
            }
            Value(v) => {
                files.push(
                    v.into_string()
                        .unwrap_or_else(|_| error::err(TOOL, "invalid UTF-8 in filename")),
                );
            }
            _ => error::err(TOOL, &format!("unexpected argument: {arg:?}")),
        }
    }

    (flags, files)
}

fn run() -> io::Result<()> {
    let (raw_flags, files) = parse_args();
    let flags = raw_flags.default_mode();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut total = Counts::default();
    let mut results: Vec<(Counts, Option<String>)> = Vec::new();

    if files.is_empty() {
        let reader = encoding::open_stdin()?;
        let c = count(reader)?;
        results.push((c, None));
    } else {
        for path in &files {
            let reader = encoding::open_input(path).unwrap_or_else(|e| {
                error::err(TOOL, &format!("{path}: {e}"));
            });
            let c = count(reader)?;
            results.push((c, Some(path.clone())));
        }
    }

    let max_val = results.iter().fold(0usize, |m, (c, _)| {
        m.max(c.lines).max(c.words).max(c.bytes).max(c.chars)
    });
    let width = format!("{max_val}").len().max(1);

    for (c, name) in &results {
        total.lines += c.lines;
        total.words += c.words;
        total.bytes += c.bytes;
        total.chars += c.chars;

        let formatted = format_counts(c, &flags, width);
        match name {
            Some(n) => writeln!(out, "{formatted} {n}")?,
            None => writeln!(out, "{formatted}")?,
        }
    }

    if results.len() > 1 {
        let formatted = format_counts(&total, &flags, width);
        writeln!(out, "{formatted} total")?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        if e.kind() != io::ErrorKind::BrokenPipe {
            error::err(TOOL, &e.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn basic_counts() {
        let input = "hello world\nfoo bar baz\n";
        let c = count(Cursor::new(input.as_bytes())).unwrap();
        assert_eq!(c.lines, 2);
        assert_eq!(c.words, 5);
        assert_eq!(c.bytes, 24);
    }

    #[test]
    fn empty_file() {
        let c = count(Cursor::new(b"" as &[u8])).unwrap();
        assert_eq!(c.lines, 0);
        assert_eq!(c.words, 0);
        assert_eq!(c.bytes, 0);
    }

    #[test]
    fn no_trailing_newline() {
        let input = "hello";
        let c = count(Cursor::new(input.as_bytes())).unwrap();
        assert_eq!(c.lines, 0);
        assert_eq!(c.words, 1);
        assert_eq!(c.bytes, 5);
    }

    #[test]
    fn utf8_multibyte() {
        let input = "cafe\u{0301}\n";
        let c = count(Cursor::new(input.as_bytes())).unwrap();
        assert_eq!(c.bytes, 7); // c-a-f-e + 2-byte combining accent + \n
        assert_eq!(c.chars, 6); // c-a-f-e-combining-\n
    }

    #[test]
    fn format_default_flags() {
        let flags = Flags {
            lines: true,
            words: true,
            bytes: true,
            chars: false,
        };
        let counts = Counts {
            lines: 5,
            words: 10,
            bytes: 50,
            chars: 45,
        };
        let result = format_counts(&counts, &flags, 3);
        assert_eq!(result, "  5  10  50");
    }

    #[test]
    fn format_single_flag() {
        let flags = Flags {
            lines: true,
            words: false,
            bytes: false,
            chars: false,
        };
        let counts = Counts {
            lines: 42,
            ..Default::default()
        };
        let result = format_counts(&counts, &flags, 3);
        assert_eq!(result, " 42");
    }

    #[test]
    fn word_count_multiple_spaces() {
        let input = "  hello   world  \n";
        let c = count(Cursor::new(input.as_bytes())).unwrap();
        assert_eq!(c.words, 2);
    }
}
