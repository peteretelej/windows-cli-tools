mod helpers;

use helpers::*;

#[test]
fn help_version() {
    check_help_version("tail");
}

#[test]
fn line_mode() {
    let f20 = fixture("twenty-lines.txt").to_str().unwrap().to_string();
    let f5 = fixture("five-lines.txt").to_str().unwrap().to_string();
    run_cases("tail", &[
        Case {
            name: "default last 10",
            args: vec![f20.clone()],
            expected: "line 11\nline 12\nline 13\nline 14\nline 15\nline 16\nline 17\nline 18\nline 19\nline 20\n",
        },
        Case {
            name: "-n 3",
            args: vec!["-n".into(), "3".into(), f20.clone()],
            expected: "line 18\nline 19\nline 20\n",
        },
        Case {
            name: "-5 shorthand",
            args: vec!["-5".into(), f20.clone()],
            expected: "line 16\nline 17\nline 18\nline 19\nline 20\n",
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
        "tail",
        &["-c", "10", fixture("twenty-lines.txt").to_str().unwrap()],
    );
    assert_exit_success(&out);
    assert_eq!(out.stdout.len(), 10);
}

#[test]
fn multi_file_headers() {
    let out = run(
        "tail",
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
    run_stdin_cases(
        "tail",
        &[StdinCase {
            name: "last 2 lines",
            args: args(&["-n", "2"]),
            stdin: b"a\nb\nc\nd\ne\n",
            expected: "d\ne\n",
        }],
    );
}

#[test]
fn encoding() {
    run_cases(
        "tail",
        &[Case {
            name: "utf16le bom",
            args: args_with_fixture(&[], "utf16le-bom.txt"),
            expected: "hello\nworld\n",
        }],
    );
    let out = run("tail", &[fixture("crlf.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    assert_stdout_contains(&out, "alpha");
    assert_stdout_contains(&out, "gamma");
}

#[test]
fn errors() {
    let out = run("tail", &["nonexistent_file_xyz.txt"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "tail:");

    let out = run("tail", &["--bogus"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "tail:");
}
