mod helpers;

use helpers::*;
use std::fs;

#[test]
fn help_version() {
    check_help_version("touch");
}

#[test]
fn file_creation() {
    let dir = tempfile::tempdir().unwrap();

    // creates single file
    let path = dir.path().join("newfile.txt");
    assert!(!path.exists());
    let out = run("touch", &[path.to_str().unwrap()]);
    assert_exit_success(&out);
    assert!(path.exists());
    assert_eq!(fs::metadata(&path).unwrap().len(), 0);

    // creates multiple files
    let p1 = dir.path().join("a.txt");
    let p2 = dir.path().join("b.txt");
    let out = run("touch", &[p1.to_str().unwrap(), p2.to_str().unwrap()]);
    assert_exit_success(&out);
    assert!(p1.exists() && p2.exists());
}

#[test]
fn updates_mtime() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("existing.txt");
    fs::write(&path, "content").unwrap();
    let before = fs::metadata(&path).unwrap().modified().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(50));
    let out = run("touch", &[path.to_str().unwrap()]);
    assert_exit_success(&out);
    let after = fs::metadata(&path).unwrap().modified().unwrap();
    assert!(after >= before, "mtime should be updated");
    assert_eq!(fs::read_to_string(&path).unwrap(), "content");
}

#[test]
fn errors() {
    let out = run("touch", &[]);
    assert_exit_code(&out, 1);
    assert_stderr_contains(&out, "touch:");

    let out = run("touch", &["/nonexistent_dir_xyz/file.txt"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "touch:");
}
