use common::{cli, encoding, error};
use lexopt::prelude::*;
use regex_lite::{Regex, RegexBuilder};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

const TOOL: &str = "grep";
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Opts {
    pattern: String,
    files: Vec<String>,
    ignore_case: bool,
    line_number: bool,
    invert: bool,
    count: bool,
    files_with_matches: bool,
    recursive: bool,
}

fn parse_args() -> Opts {
    let mut parser = lexopt::Parser::from_env();
    let mut ignore_case = false;
    let mut line_number = false;
    let mut invert = false;
    let mut count = false;
    let mut files_with_matches = false;
    let mut recursive = false;
    let mut positionals: Vec<String> = Vec::new();

    while let Some(arg) = parser
        .next()
        .unwrap_or_else(|e| error::err(TOOL, &e.to_string()))
    {
        match arg {
            Short('i') | Long("ignore-case") => ignore_case = true,
            Short('n') | Long("line-number") => line_number = true,
            Short('v') | Long("invert-match") => invert = true,
            Short('c') | Long("count") => count = true,
            Short('l') | Long("files-with-matches") => files_with_matches = true,
            Short('r') | Long("recursive") => recursive = true,
            Long("help") => {
                cli::print_help(TOOL, USAGE);
                std::process::exit(0);
            }
            Long("version") => {
                cli::print_version(TOOL, VERSION);
                std::process::exit(0);
            }
            Value(v) => positionals.push(
                v.into_string()
                    .unwrap_or_else(|_| error::err(TOOL, "invalid UTF-8 in argument")),
            ),
            _ => error::err(TOOL, &format!("unexpected argument: {arg:?}")),
        }
    }

    if positionals.is_empty() {
        error::err(TOOL, "missing PATTERN argument");
    }

    let pattern = positionals.remove(0);
    let files = positionals;

    Opts {
        pattern,
        files,
        ignore_case,
        line_number,
        invert,
        count,
        files_with_matches,
        recursive,
    }
}

const USAGE: &str = "\
Usage: grep [OPTIONS] PATTERN [FILE...]

Search for PATTERN in each FILE (or stdin).

Options:
  -i, --ignore-case        Case-insensitive matching
  -n, --line-number         Prefix matches with line number
  -v, --invert-match        Select non-matching lines
  -c, --count               Print count of matching lines per file
  -l, --files-with-matches  Print only filenames with matches
  -r, --recursive           Search directories recursively
      --help                Show this help
      --version             Show version";

fn is_binary(path: &Path) -> bool {
    let Ok(file) = fs::File::open(path) else {
        return false;
    };
    let mut buf = [0u8; 8192];
    let n = io::Read::read(&mut &file, &mut buf).unwrap_or(0);
    buf[..n].contains(&0)
}

fn collect_files(paths: &[String], recursive: bool) -> Vec<String> {
    let mut result = Vec::new();
    for p in paths {
        let path = Path::new(p);
        if path.is_dir() {
            if recursive {
                walk_dir(path, &mut result);
            } else {
                error::warn(TOOL, &format!("{p}: Is a directory"));
            }
        } else {
            result.push(p.clone());
        }
    }
    result
}

fn walk_dir(dir: &Path, out: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            error::warn(TOOL, &format!("{}: {e}", dir.display()));
            return;
        }
    };
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with('.') {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path, out);
        } else if !is_binary(&path) {
            out.push(path.to_string_lossy().into_owned());
        }
    }
}

fn search_reader(
    reader: impl BufRead,
    filename: Option<&str>,
    re: &Regex,
    opts: &Opts,
    show_prefix: bool,
    out: &mut impl Write,
) -> io::Result<bool> {
    let mut match_count: usize = 0;
    let mut found = false;

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let matches = re.is_match(&line);
        let selected = if opts.invert { !matches } else { matches };

        if selected {
            found = true;
            match_count += 1;

            if opts.files_with_matches {
                if let Some(name) = filename {
                    writeln!(out, "{name}")?;
                }
                return Ok(true);
            }

            if !opts.count {
                if show_prefix {
                    if let Some(name) = filename {
                        write!(out, "{name}:")?;
                    }
                }
                if opts.line_number {
                    write!(out, "{}:", i + 1)?;
                }
                writeln!(out, "{line}")?;
            }
        }
    }

    if opts.count {
        if show_prefix {
            if let Some(name) = filename {
                write!(out, "{name}:")?;
            }
        }
        writeln!(out, "{match_count}")?;
    }

    Ok(found)
}

fn run() -> i32 {
    let opts = parse_args();

    let re = match RegexBuilder::new(&opts.pattern)
        .case_insensitive(opts.ignore_case)
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{TOOL}: invalid regex '{}': {e}", opts.pattern);
            return 2;
        }
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut any_match = false;

    if opts.files.is_empty() {
        let reader = match encoding::open_stdin() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{TOOL}: stdin: {e}");
                return 2;
            }
        };
        match search_reader(reader, None, &re, &opts, false, &mut out) {
            Ok(found) => any_match = found,
            Err(e) if e.kind() == io::ErrorKind::BrokenPipe => return 0,
            Err(e) => {
                eprintln!("{TOOL}: {e}");
                return 2;
            }
        }
    } else {
        let files = collect_files(&opts.files, opts.recursive);
        let show_prefix = files.len() > 1;

        for path in &files {
            let reader = match encoding::open_input(path) {
                Ok(r) => r,
                Err(e) => {
                    error::warn(TOOL, &format!("{path}: {e}"));
                    continue;
                }
            };
            match search_reader(reader, Some(path), &re, &opts, show_prefix, &mut out) {
                Ok(found) => {
                    if found {
                        any_match = true;
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::BrokenPipe => return 0,
                Err(e) => {
                    error::warn(TOOL, &format!("{path}: {e}"));
                }
            }
        }
    }

    if any_match {
        0
    } else {
        1
    }
}

fn main() {
    std::process::exit(run());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    fn make_reader(s: &str) -> impl BufRead {
        BufReader::new(Cursor::new(s.to_string()))
    }

    fn default_opts() -> Opts {
        Opts {
            pattern: String::new(),
            files: vec![],
            ignore_case: false,
            line_number: false,
            invert: false,
            count: false,
            files_with_matches: false,
            recursive: false,
        }
    }

    #[test]
    fn basic_match() {
        let re = Regex::new("hello").unwrap();
        let opts = default_opts();
        let mut out = Vec::new();
        let found = search_reader(
            make_reader("hello world\nfoo\n"),
            None,
            &re,
            &opts,
            false,
            &mut out,
        )
        .unwrap();
        assert!(found);
        assert_eq!(String::from_utf8(out).unwrap(), "hello world\n");
    }

    #[test]
    fn no_match() {
        let re = Regex::new("xyz").unwrap();
        let opts = default_opts();
        let mut out = Vec::new();
        let found = search_reader(
            make_reader("hello\nworld\n"),
            None,
            &re,
            &opts,
            false,
            &mut out,
        )
        .unwrap();
        assert!(!found);
        assert_eq!(String::from_utf8(out).unwrap(), "");
    }

    #[test]
    fn case_insensitive() {
        let re = RegexBuilder::new("hello")
            .case_insensitive(true)
            .build()
            .unwrap();
        let opts = default_opts();
        let mut out = Vec::new();
        let found = search_reader(
            make_reader("HELLO\nworld\n"),
            None,
            &re,
            &opts,
            false,
            &mut out,
        )
        .unwrap();
        assert!(found);
        assert_eq!(String::from_utf8(out).unwrap(), "HELLO\n");
    }

    #[test]
    fn line_numbers() {
        let re = Regex::new("o").unwrap();
        let mut opts = default_opts();
        opts.line_number = true;
        let mut out = Vec::new();
        search_reader(
            make_reader("foo\nbar\nboo\n"),
            None,
            &re,
            &opts,
            false,
            &mut out,
        )
        .unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "1:foo\n3:boo\n");
    }

    #[test]
    fn invert_match() {
        let re = Regex::new("foo").unwrap();
        let mut opts = default_opts();
        opts.invert = true;
        let mut out = Vec::new();
        search_reader(
            make_reader("foo\nbar\nbaz\n"),
            None,
            &re,
            &opts,
            false,
            &mut out,
        )
        .unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "bar\nbaz\n");
    }

    #[test]
    fn count_mode() {
        let re = Regex::new("a").unwrap();
        let mut opts = default_opts();
        opts.count = true;
        let mut out = Vec::new();
        search_reader(
            make_reader("abc\ndef\nabc\n"),
            None,
            &re,
            &opts,
            false,
            &mut out,
        )
        .unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "2\n");
    }

    #[test]
    fn files_with_matches_mode() {
        let re = Regex::new("hello").unwrap();
        let mut opts = default_opts();
        opts.files_with_matches = true;
        let mut out = Vec::new();
        let found = search_reader(
            make_reader("hello world\nfoo\n"),
            Some("test.txt"),
            &re,
            &opts,
            true,
            &mut out,
        )
        .unwrap();
        assert!(found);
        assert_eq!(String::from_utf8(out).unwrap(), "test.txt\n");
    }

    #[test]
    fn multi_file_prefix() {
        let re = Regex::new("x").unwrap();
        let opts = default_opts();
        let mut out = Vec::new();
        search_reader(
            make_reader("ax\nby\n"),
            Some("f1.txt"),
            &re,
            &opts,
            true,
            &mut out,
        )
        .unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "f1.txt:ax\n");
    }

    #[test]
    fn count_with_prefix() {
        let re = Regex::new("a").unwrap();
        let mut opts = default_opts();
        opts.count = true;
        let mut out = Vec::new();
        search_reader(
            make_reader("abc\ndef\nabc\n"),
            Some("f.txt"),
            &re,
            &opts,
            true,
            &mut out,
        )
        .unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "f.txt:2\n");
    }

    #[test]
    fn invalid_regex() {
        let result = RegexBuilder::new("[invalid").build();
        assert!(result.is_err());
    }
}
