use lexopt::prelude::*;
use std::fs::{self, File, FileTimes};
use std::io;
use std::time::SystemTime;

const TOOL: &str = "touch";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_args() -> Vec<String> {
    let mut parser = lexopt::Parser::from_env();
    let mut files = Vec::new();

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

    if files.is_empty() {
        eprintln!("{TOOL}: missing file operand");
        std::process::exit(1);
    }

    files
}

const USAGE: &str = "\
Usage: touch FILE...

Create FILE if it does not exist, or update its modification time.

Options:
      --help    Show this help
      --version Show version";

fn touch(path: &str) -> io::Result<()> {
    if fs::metadata(path).is_ok() {
        let file = File::options().write(true).open(path)?;
        let times = FileTimes::new().set_modified(SystemTime::now());
        file.set_times(times)?;
    } else {
        File::create(path)?;
    }
    Ok(())
}

fn run() -> io::Result<()> {
    let files = parse_args();
    let mut had_error = false;

    for path in &files {
        if let Err(e) = touch(path) {
            eprintln!("{TOOL}: cannot touch '{path}': {e}");
            had_error = true;
        }
    }

    if had_error {
        std::process::exit(1);
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{TOOL}: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn create_new_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("newfile.txt");
        let path_str = path.to_str().unwrap();
        touch(path_str).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn update_existing_timestamp() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("existing.txt");
        fs::write(&path, "content").unwrap();
        let before = fs::metadata(&path).unwrap().modified().unwrap();
        thread::sleep(Duration::from_millis(50));
        touch(path.to_str().unwrap()).unwrap();
        let after = fs::metadata(&path).unwrap().modified().unwrap();
        assert!(after > before);
    }

    #[test]
    fn error_on_invalid_path() {
        let result = touch("/nonexistent/dir/file.txt");
        assert!(result.is_err());
    }
}
