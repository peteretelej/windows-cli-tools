use lexopt::prelude::*;
use std::io::{self, Write};

const TOOL: &str = "yes";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_args() -> String {
    let mut parser = lexopt::Parser::from_env();
    let mut parts: Vec<String> = Vec::new();

    while let Some(arg) = parser.next().unwrap_or_else(|e| {
        eprintln!("{TOOL}: {e}");
        std::process::exit(1);
    }) {
        match arg {
            Long("help") => {
                println!("{TOOL} - {USAGE}");
                std::process::exit(0);
            }
            Long("version") => {
                println!("{TOOL} {VERSION}");
                std::process::exit(0);
            }
            Value(v) => parts.push(v.into_string().unwrap_or_else(|_| {
                eprintln!("{TOOL}: invalid UTF-8 in argument");
                std::process::exit(1);
            })),
            _ => {
                eprintln!("{TOOL}: unexpected argument: {arg:?}");
                std::process::exit(1);
            }
        }
    }

    if parts.is_empty() {
        "y".to_string()
    } else {
        parts.join(" ")
    }
}

const USAGE: &str = "\
Usage: yes [STRING]

Repeatedly output STRING (default: 'y') followed by a newline.

Options:
      --help    Show this help
      --version Show version";

fn fill_buffer(line: &[u8], buf: &mut Vec<u8>) {
    buf.clear();
    while buf.len() + line.len() <= 8192 {
        buf.extend_from_slice(line);
    }
    if buf.is_empty() {
        buf.extend_from_slice(line);
    }
}

fn run() -> io::Result<()> {
    let text = parse_args();
    let line = format!("{text}\n");
    let line_bytes = line.as_bytes();

    let stdout = io::stdout();
    let mut out = io::BufWriter::with_capacity(8192, stdout.lock());
    let mut buf = Vec::with_capacity(8192);
    fill_buffer(line_bytes, &mut buf);

    loop {
        out.write_all(&buf)?;
    }
}

fn main() {
    if let Err(e) = run() {
        if e.kind() != io::ErrorKind::BrokenPipe {
            eprintln!("{TOOL}: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_buffer_default() {
        let line = b"y\n";
        let mut buf = Vec::with_capacity(8192);
        fill_buffer(line, &mut buf);
        assert!(buf.len() <= 8192 + line.len());
        assert!(buf.len() >= 8192);
        assert!(buf.starts_with(b"y\n"));
        assert!(buf.ends_with(b"y\n"));
    }

    #[test]
    fn fill_buffer_custom() {
        let line = b"hello world\n";
        let mut buf = Vec::with_capacity(8192);
        fill_buffer(line, &mut buf);
        assert!(buf.len() > 0);
        assert!(buf.len() % line.len() == 0);
        for chunk in buf.chunks_exact(line.len()) {
            assert_eq!(chunk, line);
        }
    }

    #[test]
    fn fill_buffer_large_line() {
        let line = vec![b'x'; 10000];
        let mut buf = Vec::new();
        fill_buffer(&line, &mut buf);
        assert_eq!(buf.len(), 10000);
    }
}
