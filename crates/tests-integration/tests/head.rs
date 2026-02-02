mod helpers;

use helpers::*;

#[test]
fn help_version() {
    check_help_version("head");
}

#[test]
fn line_mode() {
    let f20 = fixture("twenty-lines.txt").to_str().unwrap().to_string();
    let f5 = fixture("five-lines.txt").to_str().unwrap().to_string();
    run_cases("head", &[
        Case {
            name: "default 10 lines",
            args: vec![f20.clone()],
            expected: "line 1\nline 2\nline 3\nline 4\nline 5\nline 6\nline 7\nline 8\nline 9\nline 10\n",
        },
        Case {
            name: "-n 3",
            args: vec!["-n".into(), "3".into(), f20.clone()],
            expected: "line 1\nline 2\nline 3\n",
        },
        Case {
            name: "-5 shorthand",
            args: vec!["-5".into(), f20.clone()],
            expected: "line 1\nline 2\nline 3\nline 4\nline 5\n",
        },
        Case {
            name: "fewer lines than requested",
            args: vec!["-n".into(), "20".into(), f5],
            expected: "one\ntwo\nthree\nfour\nfive\n",
        },
    ]);
}

#[test]
fn byte_mode() {
    let out = run(
        "head",
        &["-c", "10", fixture("five-lines.txt").to_str().unwrap()],
    );
    assert_exit_success(&out);
    assert_eq!(out.stdout.len(), 10);
}

#[test]
fn multi_file_headers() {
    let out = run(
        "head",
        &[
            fixture("five-lines.txt").to_str().unwrap(),
            fixture("words.txt").to_str().unwrap(),
        ],
    );
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("==>"));
    assert!(s.contains("five-lines.txt"));
    assert!(s.contains("words.txt"));
}

#[test]
fn stdin() {
    let out = run_with_stdin("head", &[], b"a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\n");
    assert_exit_success(&out);
    assert_eq!(stdout_str(&out).lines().count(), 10);
}

#[test]
fn encoding() {
    run_cases(
        "head",
        &[Case {
            name: "utf16le bom",
            args: args_with_fixture(&[], "utf16le-bom.txt"),
            expected: "hello\nworld\n",
        }],
    );
    let out = run("head", &[fixture("crlf.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    assert_stdout_contains(&out, "alpha");
    assert_stdout_contains(&out, "gamma");
}

#[test]
fn errors() {
    let out = run("head", &["nonexistent_file_xyz.txt"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "head:");

    let out = run("head", &["--bogus"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "head:");
}
