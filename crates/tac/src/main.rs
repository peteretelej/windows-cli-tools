use common::cli;
use common::encoding;
use common::error;
use common::lexopt;
use common::lexopt::prelude::*;
use std::io::{self, BufRead, Write};

const TOOL: &str = "tac";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_args() -> Vec<String> {
    let mut files = Vec::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser
        .next()
        .unwrap_or_else(|e| error::err(TOOL, &e.to_string()))
    {
        match arg {
            Long("help") => {
                cli::print_help(TOOL, "concatenate and print files in reverse");
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

    files
}

fn tac(reader: impl BufRead, out: &mut impl Write) -> io::Result<()> {
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    lines.reverse();
    for line in &lines {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

fn run() -> io::Result<()> {
    let files = parse_args();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    if files.is_empty() {
        let reader = encoding::open_stdin()?;
        tac(reader, &mut out)?;
    } else {
        for path in &files {
            let reader = encoding::open_input(path).unwrap_or_else(|e| {
                error::err(TOOL, &format!("{path}: {e}"));
            });
            tac(reader, &mut out)?;
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
    fn basic_reversal() {
        let input = "one\ntwo\nthree\nfour\nfive\n";
        let mut out = Vec::new();
        tac(make_reader(input), &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines, vec!["five", "four", "three", "two", "one"]);
    }

    #[test]
    fn single_line() {
        let input = "only\n";
        let mut out = Vec::new();
        tac(make_reader(input), &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert_eq!(output.trim(), "only");
    }

    #[test]
    fn empty_file() {
        let mut out = Vec::new();
        tac(make_reader(""), &mut out).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn no_trailing_newline() {
        let input = "a\nb\nc";
        let mut out = Vec::new();
        tac(make_reader(input), &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines, vec!["c", "b", "a"]);
    }
}
