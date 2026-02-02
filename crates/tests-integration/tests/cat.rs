mod helpers;

use helpers::*;

#[test]
fn help_version() {
    check_help_version("cat");
}

#[test]
fn plain_output() {
    run_cases(
        "cat",
        &[Case {
            name: "single file",
            args: args_with_fixture(&[], "five-lines.txt"),
            expected: "one\ntwo\nthree\nfour\nfive\n",
        }],
    );
    // multi-file concat
    let out = run(
        "cat",
        &[
            fixture("five-lines.txt").to_str().unwrap(),
            fixture("words.txt").to_str().unwrap(),
        ],
    );
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.starts_with("one\n"));
    assert!(s.contains("the lazy dog\n"));
}

#[test]
fn line_numbering() {
    let out = run("cat", &["-n", fixture("five-lines.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("     1\tone"));
    assert!(s.contains("     5\tfive"));

    // continuous numbering across files
    let out = run(
        "cat",
        &[
            "-n",
            fixture("five-lines.txt").to_str().unwrap(),
            fixture("words.txt").to_str().unwrap(),
        ],
    );
    assert_exit_success(&out);
    assert_stdout_contains(&out, "     6\tthe quick brown fox");
}

#[test]
fn raw_mode() {
    let out = run("cat", &["--raw", fixture("binary.bin").to_str().unwrap()]);
    assert_exit_success(&out);
    assert_eq!(out.stdout.len(), 10);
    assert_eq!(out.stdout[0], 0x00);

    // raw preserves UTF-16 BOM bytes
    let out = run(
        "cat",
        &["--raw", fixture("utf16le-bom.txt").to_str().unwrap()],
    );
    assert_exit_success(&out);
    assert_eq!(out.stdout[0], 0xFF);
    assert_eq!(out.stdout[1], 0xFE);
}

#[test]
fn stdin_and_encoding() {
    run_stdin_cases(
        "cat",
        &[StdinCase {
            name: "stdin passthrough",
            args: vec![],
            stdin: b"hello world\n",
            expected: "hello world\n",
        }],
    );
    run_cases(
        "cat",
        &[Case {
            name: "utf16le bom transcoded",
            args: args_with_fixture(&[], "utf16le-bom.txt"),
            expected: "hello\nworld\n",
        }],
    );
}

#[test]
fn missing_file() {
    let out = run("cat", &["nonexistent_file_xyz.txt"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "cat:");
}
