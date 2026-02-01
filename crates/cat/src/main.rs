use common::cli;
use common::encoding;
use common::error;
use common::lexopt;
use common::lexopt::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Write};

const TOOL: &str = "cat";
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Opts {
    number: bool,
    raw: bool,
    files: Vec<String>,
}

fn parse_args() -> Opts {
    let mut opts = Opts {
        number: false,
        raw: false,
        files: Vec::new(),
    };
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser
        .next()
        .unwrap_or_else(|e| error::err(TOOL, &e.to_string()))
    {
        match arg {
            Short('n') | Long("number") => opts.number = true,
            Long("raw") => opts.raw = true,
            Long("help") => {
                cli::print_help(TOOL, "concatenate files and print on the standard output");
                std::process::exit(0);
            }
            Long("version") => {
                cli::print_version(TOOL, VERSION);
                std::process::exit(0);
            }
            Value(v) => {
                opts.files.push(
                    v.into_string()
                        .unwrap_or_else(|_| error::err(TOOL, "invalid UTF-8 in filename")),
                );
            }
            _ => error::err(TOOL, &format!("unexpected argument: {arg:?}")),
        }
    }

    opts
}

fn cat_numbered(
    reader: impl BufRead,
    line_num: &mut usize,
    out: &mut impl Write,
) -> io::Result<()> {
    for line in reader.lines() {
        let line = line?;
        writeln!(out, "{:>6}\t{line}", line_num)?;
        *line_num += 1;
    }
    Ok(())
}

fn cat_plain(mut reader: impl BufRead, out: &mut impl Write) -> io::Result<()> {
    io::copy(&mut reader, out)?;
    Ok(())
}

fn cat_raw(path: &str, out: &mut impl Write) -> io::Result<()> {
    let mut file = File::open(path)?;
    io::copy(&mut file, out)?;
    Ok(())
}

fn run() -> io::Result<()> {
    let opts = parse_args();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut line_num: usize = 1;

    if opts.files.is_empty() {
        let reader = encoding::open_stdin()?;
        if opts.number {
            cat_numbered(reader, &mut line_num, &mut out)?;
        } else {
            cat_plain(reader, &mut out)?;
        }
    } else {
        for path in &opts.files {
            if opts.raw {
                cat_raw(path, &mut out).unwrap_or_else(|e| {
                    error::err(TOOL, &format!("{path}: {e}"));
                });
            } else {
                let reader = encoding::open_input(path).unwrap_or_else(|e| {
                    error::err(TOOL, &format!("{path}: {e}"));
                });
                if opts.number {
                    cat_numbered(reader, &mut line_num, &mut out)?;
                } else {
                    cat_plain(reader, &mut out)?;
                }
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
    fn plain_output() {
        let input = "hello\nworld\n";
        let mut out = Vec::new();
        cat_plain(make_reader(input), &mut out).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), input);
    }

    #[test]
    fn numbered_output() {
        let input = "alpha\nbeta\ngamma\n";
        let mut out = Vec::new();
        let mut n = 1;
        cat_numbered(make_reader(input), &mut n, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert!(output.contains("     1\talpha"));
        assert!(output.contains("     2\tbeta"));
        assert!(output.contains("     3\tgamma"));
        assert_eq!(n, 4);
    }

    #[test]
    fn continuous_numbering() {
        let mut out = Vec::new();
        let mut n = 1;
        cat_numbered(make_reader("a\nb\n"), &mut n, &mut out).unwrap();
        cat_numbered(make_reader("c\nd\n"), &mut n, &mut out).unwrap();
        let output = String::from_utf8(out).unwrap();
        assert!(output.contains("     3\tc"));
        assert!(output.contains("     4\td"));
    }

    #[test]
    fn empty_file() {
        let mut out = Vec::new();
        cat_plain(make_reader(""), &mut out).unwrap();
        assert!(out.is_empty());
    }

    #[test]
    fn empty_file_numbered() {
        let mut out = Vec::new();
        let mut n = 1;
        cat_numbered(make_reader(""), &mut n, &mut out).unwrap();
        assert!(out.is_empty());
        assert_eq!(n, 1);
    }
}
