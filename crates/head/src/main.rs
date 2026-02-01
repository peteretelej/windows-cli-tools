use common::cli;
use common::encoding;
use common::error;
use common::lexopt;
use common::lexopt::prelude::*;
use std::io::{self, BufRead, Read, Write};

const TOOL: &str = "head";
const VERSION: &str = env!("CARGO_PKG_VERSION");

enum Mode {
    Lines(usize),
    Bytes(usize),
}

fn parse_args() -> (Mode, Vec<String>) {
    let mut mode = Mode::Lines(10);
    let mut files = Vec::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser
        .next()
        .unwrap_or_else(|e| error::err(TOOL, &e.to_string()))
    {
        match arg {
            Short('n') | Long("lines") => {
                let val: usize = parser
                    .value()
                    .unwrap_or_else(|e| error::err(TOOL, &e.to_string()))
                    .parse()
                    .unwrap_or_else(|e| error::err(TOOL, &e.to_string()));
                mode = Mode::Lines(val);
            }
            Short('c') | Long("bytes") => {
                let val: usize = parser
                    .value()
                    .unwrap_or_else(|e| error::err(TOOL, &e.to_string()))
                    .parse()
                    .unwrap_or_else(|e| error::err(TOOL, &e.to_string()));
                mode = Mode::Bytes(val);
            }
            Long("help") => {
                cli::print_help(TOOL, "output the first part of files");
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

    (mode, files)
}

fn head_lines(reader: impl BufRead, n: usize, out: &mut impl Write) -> io::Result<()> {
    for line in reader.lines().take(n) {
        let line = line?;
        writeln!(out, "{line}")?;
    }
    Ok(())
}

fn head_bytes(reader: impl Read, n: usize, out: &mut impl Write) -> io::Result<()> {
    let mut buf = vec![0u8; n];
    let mut reader = reader.take(n as u64);
    let bytes_read = reader.read(&mut buf)?;
    out.write_all(&buf[..bytes_read])?;
    Ok(())
}

fn run() -> io::Result<()> {
    let (mode, files) = parse_args();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let multiple = files.len() > 1;

    if files.is_empty() {
        let reader = encoding::open_stdin()?;
        match &mode {
            Mode::Lines(n) => head_lines(reader, *n, &mut out)?,
            Mode::Bytes(n) => head_bytes(reader, *n, &mut out)?,
        }
    } else {
        for (i, path) in files.iter().enumerate() {
            if multiple {
                if i > 0 {
                    writeln!(out)?;
                }
                writeln!(out, "==> {path} <==")?;
            }
            let reader = encoding::open_input(path).unwrap_or_else(|e| {
                error::err(TOOL, &format!("{path}: {e}"));
            });
            match &mode {
                Mode::Lines(n) => head_lines(reader, *n, &mut out)?,
                Mode::Bytes(n) => head_bytes(reader, *n, &mut out)?,
            }
        }
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
    use std::io::{BufReader, Cursor};

    use super::*;

    fn make_reader(s: &str) -> impl BufRead {
        BufReader::new(Cursor::new(s.to_string()))
    }

    #[test]
    fn default_10_lines() {
        let input = (1..=20)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut out = Vec::new();
        head_lines(make_reader(&input), 10, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.lines().count(), 10);
        assert!(output.starts_with("line 1\n"));
        assert!(output.contains("line 10\n"));
    }

    #[test]
    fn custom_line_count() {
        let input = (1..=10)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut out = Vec::new();
        head_lines(make_reader(&input), 5, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.lines().count(), 5);
        assert!(output.ends_with("line 5\n"));
    }

    #[test]
    fn byte_mode() {
        let input = "hello world\n";
        let mut out = Vec::new();
        head_bytes(Cursor::new(input.as_bytes()), 5, &mut out).unwrap();
        assert_eq!(out, b"hello");
    }

    #[test]
    fn empty_file() {
        let mut out = Vec::new();
        head_lines(make_reader(""), 10, &mut out).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn fewer_lines_than_n() {
        let input = "one\ntwo\nthree\n";
        let mut out = Vec::new();
        head_lines(make_reader(input), 10, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.lines().count(), 3);
    }
}
