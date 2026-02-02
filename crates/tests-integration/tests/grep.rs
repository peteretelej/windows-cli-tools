mod helpers;

use helpers::*;

#[test]
fn help_version() {
    check_help_version("grep");
}

#[test]
fn basic_matching() {
    run_cases(
        "grep",
        &[Case {
            name: "simple pattern",
            args: args_with_fixture(&["quick"], "words.txt"),
            expected: "the quick brown fox\n",
        }],
    );
    // regex with multiple matches
    let out = run("grep", &["^the", fixture("words.txt").to_str().unwrap()]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert_eq!(s.lines().count(), 2);
}

#[test]
fn flags() {
    let f = fixture("words.txt").to_str().unwrap().to_string();
    let cases: &[(&str, &[&str], &str)] = &[
        (
            "case insensitive -i",
            &["-i", "THE"],
            "the quick brown fox\nthe lazy dog\n",
        ),
        ("line numbers -n", &["-n", "over"], "2:jumps over\n"),
        ("invert -v", &["-v", "the"], "jumps over\n"),
        ("count -c", &["-c", "the"], "2\n"),
    ];
    for (name, flag_args, expected) in cases {
        let mut a: Vec<String> = flag_args.iter().map(|s| s.to_string()).collect();
        a.push(f.clone());
        run_cases(
            "grep",
            &[Case {
                name,
                args: a,
                expected,
            }],
        );
    }

    // -l prints filename
    let out = run("grep", &["-l", "quick", &f]);
    assert_exit_success(&out);
    assert_stdout_contains(&out, "words.txt");

    // combined -in
    let out = run("grep", &["-i", "-n", "THE", &f]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("1:") && s.contains("3:"));
}

#[test]
fn multi_file() {
    let f1 = fixture("words.txt").to_str().unwrap().to_string();
    let f2 = fixture("five-lines.txt").to_str().unwrap().to_string();

    let out = run("grep", &["the", &f1, &f2]);
    assert_exit_success(&out);
    assert_stdout_contains(&out, "words.txt:");

    let out = run("grep", &["-c", "the", &f1, &f2]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("words.txt:2"));
    assert!(s.contains("five-lines.txt:0"));

    let out = run("grep", &["-l", "quick", &f1, &f2]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("words.txt"));
    assert!(!s.contains("five-lines.txt"));
}

#[test]
fn recursive() {
    let dir = fixture("nested").to_str().unwrap().to_string();
    let out = run("grep", &["-r", "hello", &dir]);
    assert_exit_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("a.txt"));
    assert!(s.contains("b.txt"));
    assert!(!s.contains(".hidden.txt"), "should skip hidden files");
}

#[test]
fn stdin() {
    run_stdin_cases(
        "grep",
        &[StdinCase {
            name: "stdin no prefix",
            args: args(&["fox"]),
            stdin: b"the quick brown fox\njumps over\n",
            expected: "the quick brown fox\n",
        }],
    );
}

#[test]
fn exit_codes() {
    let f = fixture("five-lines.txt").to_str().unwrap().to_string();
    assert_exit_code(&run("grep", &["one", &f]), 0);
    assert_exit_code(&run("grep", &["zzz", &f]), 1);

    let out = run("grep", &["[invalid", &f]);
    assert_exit_code(&out, 2);
    assert_stderr_contains(&out, "grep:");
}

#[test]
fn utf16le_bom() {
    let out = run(
        "grep",
        &["hello", fixture("utf16le-bom.txt").to_str().unwrap()],
    );
    assert_exit_success(&out);
    assert_stdout_contains(&out, "hello");
}
