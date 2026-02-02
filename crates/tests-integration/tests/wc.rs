mod helpers;

use helpers::*;

#[test]
fn help_version() {
    check_help_version("wc");
}

#[test]
fn default_and_empty() {
    let out = run("wc", &[fixture("five-lines.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("5"), "expected 5 lines in: {s:?}");
    assert!(s.contains("five-lines.txt"));

    let out = run("wc", &[fixture("empty.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    assert_stdout_contains(&out, "0");
}

#[test]
fn individual_flags() {
    // five-lines.txt: "one\ntwo\nthree\nfour\nfive\n" = 5 lines, 5 words, 24 bytes
    // words.txt: "the quick brown fox\njumps over\nthe lazy dog\n" = 3 lines, 9 words
    let cases: &[(&str, &str, &str)] = &[
        ("-l", "five-lines.txt", "5"),
        ("-w", "words.txt", "9"),
        ("-c", "five-lines.txt", "24"),
        ("-m", "five-lines.txt", "24"), // ASCII: chars == bytes
    ];
    for (flag, file, expected_num) in cases {
        let out = run("wc", &[flag, fixture(file).to_str().unwrap()]);
        assert_exit_success(&out);
        let s = stdout_str(&out);
        assert!(
            s.contains(expected_num),
            "[wc {flag} {file}] expected {expected_num} in: {s:?}"
        );
    }
}

#[test]
fn combined_flags() {
    let out = run("wc", &["-l", "-w", fixture("words.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("3"), "expected 3 lines in: {s:?}");
    assert!(s.contains("9"), "expected 9 words in: {s:?}");
}

#[test]
fn multi_file_totals() {
    let out = run(
        "wc",
        &[
            fixture("five-lines.txt").to_str().unwrap(),
            fixture("words.txt").to_str().unwrap(),
        ],
    );
    assert_exit_success(&out);
    assert_stdout_contains(&out, "total");
}

#[test]
fn stdin_no_filename() {
    let out = run_with_stdin("wc", &[], b"one two three\nfour five\n");
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("2"), "expected 2 lines in: {s:?}");
    assert!(s.contains("5"), "expected 5 words in: {s:?}");
    assert!(!s.contains(".txt"));
}

#[test]
fn utf16le_bom() {
    let out = run("wc", &[fixture("utf16le-bom.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    assert_stdout_contains(&out, "2");
}

#[test]
fn missing_file() {
    let out = run("wc", &["nonexistent_file_xyz.txt"]);
    assert!(!out.status.success());
    assert_stderr_contains(&out, "wc:");
}
