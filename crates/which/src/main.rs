use lexopt::prelude::*;
use std::env;
use std::path::{Path, PathBuf};

const TOOL: &str = "which";
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Opts {
    command: String,
    all: bool,
}

fn parse_args() -> Opts {
    let mut parser = lexopt::Parser::from_env();
    let mut all = false;
    let mut command: Option<String> = None;

    while let Some(arg) = parser.next().unwrap_or_else(|e| {
        eprintln!("{TOOL}: {e}");
        std::process::exit(1);
    }) {
        match arg {
            Short('a') | Long("all") => all = true,
            Long("help") => {
                println!("{TOOL} - {USAGE}");
                std::process::exit(0);
            }
            Long("version") => {
                println!("{TOOL} {VERSION}");
                std::process::exit(0);
            }
            Value(v) => {
                if command.is_some() {
                    eprintln!("{TOOL}: too many arguments");
                    std::process::exit(1);
                }
                command = Some(v.into_string().unwrap_or_else(|_| {
                    eprintln!("{TOOL}: invalid UTF-8 in argument");
                    std::process::exit(1);
                }));
            }
            _ => {
                eprintln!("{TOOL}: unexpected argument: {arg:?}");
                std::process::exit(1);
            }
        }
    }

    let command = match command {
        Some(c) => c,
        None => {
            eprintln!("{TOOL}: missing command argument");
            std::process::exit(1);
        }
    };

    Opts { command, all }
}

const USAGE: &str = "\
Usage: which [OPTIONS] COMMAND

Locate an executable on PATH.

Options:
  -a, --all     Show all matches, not just the first
      --help    Show this help
      --version Show version";

fn get_extensions() -> Vec<String> {
    let pathext = env::var("PATHEXT")
        .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC".into());
    pathext
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn has_extension(name: &str) -> bool {
    Path::new(name).extension().is_some()
}

fn check_candidate(dir: &Path, name: &str, extensions: &[String]) -> Vec<PathBuf> {
    let mut found = Vec::new();

    let exact = dir.join(name);
    if exact.is_file() {
        found.push(exact);
    }

    if !has_extension(name) {
        for ext in extensions {
            let with_ext = dir.join(format!("{name}{}", ext.to_lowercase()));
            if with_ext.is_file() {
                found.push(with_ext);
            }
            if ext.to_lowercase() != ext.to_uppercase() {
                let with_ext_upper = dir.join(format!("{name}{ext}"));
                if with_ext_upper.is_file() && !found.iter().any(|f| f == &with_ext_upper) {
                    found.push(with_ext_upper);
                }
            }
        }
    }

    found
}

fn find_command(command: &str, all: bool) -> Vec<PathBuf> {
    let extensions = get_extensions();
    let mut results = Vec::new();

    if let Ok(cwd) = env::current_dir() {
        for path in check_candidate(&cwd, command, &extensions) {
            results.push(path);
            if !all {
                return results;
            }
        }
    }

    let path_var = env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ';' } else { ':' };
    for dir in path_var.split(sep) {
        if dir.is_empty() {
            continue;
        }
        let dir_path = Path::new(dir);
        for path in check_candidate(dir_path, command, &extensions) {
            if !results.iter().any(|r| r == &path) {
                results.push(path);
                if !all {
                    return results;
                }
            }
        }
    }

    results
}

fn main() {
    let opts = parse_args();
    let results = find_command(&opts.command, opts.all);

    if results.is_empty() {
        eprintln!("{TOOL}: {}: not found", opts.command);
        std::process::exit(1);
    }

    for path in &results {
        println!("{}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn find_in_directory() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("testcmd.exe");
        fs::write(&exe, "fake").unwrap();

        let extensions = vec![".EXE".to_string()];
        let found = check_candidate(dir.path(), "testcmd", &extensions);
        assert!(!found.is_empty());
        assert!(found[0].ends_with("testcmd.exe"));
    }

    #[test]
    fn exact_name_match() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("script.bat");
        fs::write(&exe, "fake").unwrap();

        let extensions = vec![".EXE".to_string()];
        let found = check_candidate(dir.path(), "script.bat", &extensions);
        assert!(!found.is_empty());
    }

    #[test]
    fn not_found() {
        let dir = tempfile::tempdir().unwrap();
        let extensions = vec![".EXE".to_string()];
        let found = check_candidate(dir.path(), "nonexistent", &extensions);
        assert!(found.is_empty());
    }

    #[test]
    fn has_extension_check() {
        assert!(has_extension("foo.exe"));
        assert!(!has_extension("foo"));
    }
}
