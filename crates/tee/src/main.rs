use lexopt::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

const TOOL: &str = "tee";
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Opts {
    files: Vec<String>,
    append: bool,
}

fn parse_args() -> Opts {
    let mut parser = lexopt::Parser::from_env();
    let mut files = Vec::new();
    let mut append = false;

    while let Some(arg) = parser.next().unwrap_or_else(|e| {
        eprintln!("{TOOL}: {e}");
        std::process::exit(1);
    }) {
        match arg {
            Short('a') | Long("append") => append = true,
            Long("help") => {
                println!("{TOOL} - {USAGE}");
                std::process::exit(0);
            }
            Long("version") => {
                println!("{TOOL} {VERSION}");
                std::process::exit(0);
            }
            Value(v) => files.push(v.into_string().unwrap_or_else(|_| {
                eprintln!("{TOOL}: invalid UTF-8 in argument");
                std::process::exit(1);
            })),
            _ => {
                eprintln!("{TOOL}: unexpected argument: {arg:?}");
                std::process::exit(1);
            }
        }
    }

    Opts { files, append }
}

const USAGE: &str = "\
Usage: tee [OPTIONS] [FILE...]

Copy stdin to stdout and each FILE.

Options:
  -a, --append  Append to files instead of overwriting
      --help    Show this help
      --version Show version";

fn run() -> io::Result<()> {
    let opts = parse_args();

    let mut outputs: Vec<Box<dyn Write>> = Vec::new();
    for path in &opts.files {
        let file = if opts.append {
            OpenOptions::new().create(true).append(true).open(path)?
        } else {
            File::create(path)?
        };
        outputs.push(Box::new(file));
    }

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buf = [0u8; 8192];

    loop {
        let n = stdin.lock().read(&mut buf)?;
        if n == 0 {
            break;
        }
        stdout.write_all(&buf[..n])?;
        stdout.flush()?;
        for out in &mut outputs {
            out.write_all(&buf[..n])?;
            out.flush()?;
        }
    }

    Ok(())
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
    use std::io::{Cursor, Read, Write};

    fn tee_core(input: &[u8], num_outputs: usize) -> (Vec<u8>, Vec<Vec<u8>>) {
        let mut stdout_buf = Vec::new();
        let mut file_bufs: Vec<Vec<u8>> = (0..num_outputs).map(|_| Vec::new()).collect();

        let mut reader = Cursor::new(input);
        let mut buf = [0u8; 8192];
        loop {
            let n = reader.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            stdout_buf.write_all(&buf[..n]).unwrap();
            for fb in &mut file_bufs {
                fb.write_all(&buf[..n]).unwrap();
            }
        }

        (stdout_buf, file_bufs)
    }

    #[test]
    fn passthrough_no_files() {
        let (stdout, files) = tee_core(b"hello world\n", 0);
        assert_eq!(stdout, b"hello world\n");
        assert!(files.is_empty());
    }

    #[test]
    fn write_to_one_file() {
        let (stdout, files) = tee_core(b"data\n", 1);
        assert_eq!(stdout, b"data\n");
        assert_eq!(files[0], b"data\n");
    }

    #[test]
    fn write_to_multiple_files() {
        let (stdout, files) = tee_core(b"abc\n", 3);
        assert_eq!(stdout, b"abc\n");
        for f in &files {
            assert_eq!(f, b"abc\n");
        }
    }
}
