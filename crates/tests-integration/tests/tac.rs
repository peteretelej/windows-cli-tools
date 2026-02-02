mod helpers;

use helpers::*;

#[test]
fn help_version() {
    check_help_version("tac");
}

#[test]
fn reversal() {
    run_cases(
        "tac",
        &[
            Case {
                name: "five lines reversed",
                args: args_with_fixture(&[], "five-lines.txt"),
                expected: "five\nfour\nthree\ntwo\none\n",
            },
            Case {
                name: "empty file",
                args: args_with_fixture(&[], "empty.txt"),
                expected: "",
            },
        ],
    );
    run_stdin_cases(
        "tac",
        &[
            StdinCase {
                name: "single line",
                args: vec![],
                stdin: b"only\n",
                expected: "only\n",
            },
            StdinCase {
                name: "stdin reversed",
                args: vec![],
                stdin: b"a\nb\nc\n",
                expected: "c\nb\na\n",
            },
        ],
    );
}

#[test]
fn multi_file() {
    let out = run(
        "tac",
        &[
            fixture("five-lines.txt").to_str().unwrap(),
            fixture("words.txt").to_str().unwrap(),
        ],
    );
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.starts_with("five\n"));
    assert!(s.ends_with("the lazy dog\njumps over\nthe quick brown fox\n"));
}

#[test]
fn utf16le_bom() {
    run_cases(
        "tac",
        &[Case {
            name: "utf16le reversed",
            args: args_with_fixture(&[], "utf16le-bom.txt"),
            expected: "world\nhello\n",
        }],
    );
}

#[test]
fn missing_file() {
    let out = run("tac", &["nonexistent_file_xyz.txt"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "tac:");
}
