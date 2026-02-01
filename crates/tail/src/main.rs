use common::cli;
use common::encoding;
use common::error;
use common::lexopt;
use common::lexopt::prelude::*;
use std::collections::VecDeque;
use std::io::{self, BufRead, Read, Write};

const TOOL: &str = "tail";
const VERSION: &str = env!("CARGO_PKG_VERSION");

enum Mode {
    Lines(usize),
    Bytes(usize),
}

fn expand_dash_n(args: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for arg in args {
        if arg.len() >= 2 && arg.starts_with('-') && arg[1..].bytes().all(|b| b.is_ascii_digit()) {
            out.push("-n".to_string());
            out.push(arg[1..].to_string());
        } else {
            out.push(arg);
        }
    }
    out
}

fn parse_args() -> (Mode, Vec<String>) {
    let mut mode = Mode::Lines(10);
    let mut files = Vec::new();
    let raw_args: Vec<String> = std::env::args().collect();
    let expanded = expand_dash_n(raw_args[1..].to_vec());
    let mut parser = lexopt::Parser::from_args(expanded);

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
                cli::print_help(TOOL, "output the last part of files");
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

fn tail_lines_buffered(reader: impl BufRead, n: usize, out: &mut impl Write) -> io::Result<()> {
    let mut ring: VecDeque<String> = VecDeque::with_capacity(n);
    for line in reader.lines() {
        let line = line?;
        if ring.len() == n {
            ring.pop_front();
        }
        ring.push_back(line);
    }
    for line in &ring {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

fn tail_bytes_buffered(mut reader: impl Read, n: usize, out: &mut impl Write) -> io::Result<()> {
    let mut all = Vec::new();
    reader.read_to_end(&mut all)?;
    let start = all.len().saturating_sub(n);
    out.write_all(&all[start..])?;
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
            Mode::Lines(n) => tail_lines_buffered(reader, *n, &mut out)?,
            Mode::Bytes(n) => tail_bytes_buffered(reader, *n, &mut out)?,
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
                Mode::Lines(n) => tail_lines_buffered(reader, *n, &mut out)?,
                Mode::Bytes(n) => tail_bytes_buffered(reader, *n, &mut out)?,
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
        tail_lines_buffered(make_reader(&input), 10, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.lines().count(), 10);
        assert!(output.starts_with("line 11\n"));
        assert!(output.contains("line 20\n"));
    }

    #[test]
    fn custom_line_count() {
        let input = (1..=10)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut out = Vec::new();
        tail_lines_buffered(make_reader(&input), 3, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.lines().count(), 3);
        assert!(output.starts_with("line 8\n"));
    }

    #[test]
    fn byte_mode() {
        let input = "hello world\n";
        let mut out = Vec::new();
        tail_bytes_buffered(Cursor::new(input.as_bytes()), 6, &mut out).unwrap();
        assert_eq!(out, b"world\n");
    }

    #[test]
    fn small_file() {
        let input = "one\ntwo\n";
        let mut out = Vec::new();
        tail_lines_buffered(make_reader(input), 10, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.lines().count(), 2);
    }

    #[test]
    fn empty_file() {
        let mut out = Vec::new();
        tail_lines_buffered(make_reader(""), 10, &mut out).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn expand_dash_number() {
        let args = vec!["-5".to_string(), "file.txt".to_string()];
        let expanded = expand_dash_n(args);
        assert_eq!(expanded, vec!["-n", "5", "file.txt"]);
    }

    #[test]
    fn expand_dash_number_no_transform_flags() {
        let args = vec!["-n".to_string(), "10".to_string()];
        let expanded = expand_dash_n(args);
        assert_eq!(expanded, vec!["-n", "10"]);
    }

    #[test]
    fn stdin_ring_buffer() {
        let input = (1..=100)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut out = Vec::new();
        tail_lines_buffered(make_reader(&input), 5, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.lines().count(), 5);
        assert!(output.starts_with("line 96\n"));
    }
}
