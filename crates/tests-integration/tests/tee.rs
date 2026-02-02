mod helpers;

use helpers::*;
use std::fs;

#[test]
fn help_version() {
    check_help_version("tee");
}

#[test]
fn basic_operation() {
    // stdout passthrough
    let out = run_with_stdin("tee", &[], b"hello world\n");
    assert_exit_success(&out);
    assert_stdout(&out, "hello world\n");

    // writes to single file
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("out.txt");
    let out = run_with_stdin("tee", &[path.to_str().unwrap()], b"hello\n");
    assert_exit_success(&out);
    assert_stdout(&out, "hello\n");
    assert_eq!(fs::read_to_string(&path).unwrap(), "hello\n");

    // writes to multiple files
    let p1 = dir.path().join("a.txt");
    let p2 = dir.path().join("b.txt");
    let out = run_with_stdin(
        "tee",
        &[p1.to_str().unwrap(), p2.to_str().unwrap()],
        b"data\n",
    );
    assert_exit_success(&out);
    assert_eq!(fs::read_to_string(&p1).unwrap(), "data\n");
    assert_eq!(fs::read_to_string(&p2).unwrap(), "data\n");
}

#[test]
fn append_mode() {
    let dir = tempfile::tempdir().unwrap();

    // append to existing file
    let path = dir.path().join("out.txt");
    fs::write(&path, "existing\n").unwrap();
    let out = run_with_stdin("tee", &["-a", path.to_str().unwrap()], b"appended\n");
    assert_exit_success(&out);
    assert_eq!(fs::read_to_string(&path).unwrap(), "existing\nappended\n");

    // append creates nonexistent file
    let new_path = dir.path().join("new.txt");
    let out = run_with_stdin("tee", &["-a", new_path.to_str().unwrap()], b"created\n");
    assert_exit_success(&out);
    assert_eq!(fs::read_to_string(&new_path).unwrap(), "created\n");
}
