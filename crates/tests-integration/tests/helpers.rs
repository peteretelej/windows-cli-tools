use std::path::PathBuf;
use std::process::{Command, Output};

#[allow(dead_code)]
pub fn bin_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // crates/
    path.pop(); // workspace root
    path.push("target");
    path.push("debug");
    path.push(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    path
}

#[allow(dead_code)]
pub fn fixture(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path
}

#[allow(dead_code)]
pub fn run(tool: &str, args: &[&str]) -> Output {
    Command::new(bin_path(tool))
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {tool}: {e}"))
}

#[allow(dead_code)]
pub fn run_with_stdin(tool: &str, args: &[&str], input: &[u8]) -> Output {
    use std::io::Write;
    let mut child = Command::new(bin_path(tool))
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn {tool}: {e}"));
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[allow(dead_code)]
pub fn stdout_str(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[allow(dead_code)]
pub fn stderr_str(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[allow(dead_code)]
pub fn assert_stdout(output: &Output, expected: &str) {
    let actual = stdout_str(output);
    assert_eq!(actual, expected, "stdout mismatch");
}

#[allow(dead_code)]
pub fn assert_stdout_contains(output: &Output, substring: &str) {
    let actual = stdout_str(output);
    assert!(
        actual.contains(substring),
        "stdout does not contain {substring:?}\nactual: {actual:?}"
    );
}

#[allow(dead_code)]
pub fn assert_stderr_contains(output: &Output, substring: &str) {
    let actual = stderr_str(output);
    assert!(
        actual.contains(substring),
        "stderr does not contain {substring:?}\nactual: {actual:?}"
    );
}

#[allow(dead_code)]
pub fn assert_exit_success(output: &Output) {
    assert!(
        output.status.success(),
        "expected exit 0, got {:?}\nstderr: {}",
        output.status.code(),
        stderr_str(output)
    );
}

#[allow(dead_code)]
pub fn assert_exit_code(output: &Output, code: i32) {
    assert_eq!(
        output.status.code(),
        Some(code),
        "expected exit code {code}\nstderr: {}",
        stderr_str(output)
    );
}

// Table-driven test case: run tool with args, assert exact stdout.
#[allow(dead_code)]
pub struct Case {
    pub name: &'static str,
    pub args: Vec<String>,
    pub expected: &'static str,
}

#[allow(dead_code)]
pub fn run_cases(tool: &str, cases: &[Case]) {
    for c in cases {
        let args: Vec<&str> = c.args.iter().map(|s| s.as_str()).collect();
        let out = run(tool, &args);
        assert!(
            out.status.success(),
            "[{}] exit {:?}\nstderr: {}",
            c.name,
            out.status.code(),
            stderr_str(&out)
        );
        let actual = stdout_str(&out);
        assert_eq!(actual, c.expected, "[{}] stdout mismatch", c.name);
    }
}

// Table-driven: run tool with stdin, assert exact stdout.
#[allow(dead_code)]
pub struct StdinCase {
    pub name: &'static str,
    pub args: Vec<String>,
    pub stdin: &'static [u8],
    pub expected: &'static str,
}

#[allow(dead_code)]
pub fn run_stdin_cases(tool: &str, cases: &[StdinCase]) {
    for c in cases {
        let args: Vec<&str> = c.args.iter().map(|s| s.as_str()).collect();
        let out = run_with_stdin(tool, &args, c.stdin);
        assert!(
            out.status.success(),
            "[{}] exit {:?}\nstderr: {}",
            c.name,
            out.status.code(),
            stderr_str(&out)
        );
        let actual = stdout_str(&out);
        assert_eq!(actual, c.expected, "[{}] stdout mismatch", c.name);
    }
}

// Verify --help and --version for a tool.
#[allow(dead_code)]
pub fn check_help_version(tool: &str) {
    let help = run(tool, &["--help"]);
    assert_exit_success(&help);
    assert_stdout_contains(&help, tool);

    let ver = run(tool, &["--version"]);
    assert_exit_success(&ver);
    assert_stdout_contains(&ver, tool);
}

// Helper: build args vec mixing &str literals and fixture paths.
#[allow(dead_code)]
pub fn args(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

// Helper: build args with a fixture path inserted.
#[allow(dead_code)]
pub fn args_with_fixture(prefix: &[&str], fixture_name: &str) -> Vec<String> {
    let mut v: Vec<String> = prefix.iter().map(|s| s.to_string()).collect();
    v.push(fixture(fixture_name).to_str().unwrap().to_string());
    v
}

// Helper: build args with multiple fixture paths appended.
#[allow(dead_code)]
pub fn args_with_fixtures(prefix: &[&str], fixtures: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = prefix.iter().map(|s| s.to_string()).collect();
    for f in fixtures {
        v.push(fixture(f).to_str().unwrap().to_string());
    }
    v
}

// Pipe one tool's output into another, returning the second tool's Output.
#[allow(dead_code)]
pub fn pipe(tool1: &str, args1: &[&str], tool2: &str, args2: &[&str]) -> Output {
    let mut child1 = Command::new(bin_path(tool1))
        .args(args1)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("failed to spawn {tool1}: {e}"));
    let stdout1 = child1.stdout.take().unwrap();
    let out = Command::new(bin_path(tool2))
        .args(args2)
        .stdin(stdout1)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {tool2}: {e}"));
    let _ = child1.wait();
    out
}
