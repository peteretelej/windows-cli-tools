mod helpers;

use helpers::*;

#[test]
fn help_version() {
    check_help_version("yes");
}

#[test]
fn output_variations() {
    let cases: &[(&str, &[&str], &str, &str)] = &[
        ("default y", &[], "-5", "y\ny\ny\ny\ny\n"),
        ("custom string", &["hello"], "-3", "hello\nhello\nhello\n"),
        (
            "multi word",
            &["hello", "world"],
            "-2",
            "hello world\nhello world\n",
        ),
    ];
    for (name, yes_args, head_arg, expected) in cases {
        let out = pipe("yes", yes_args, "head", &[head_arg]);
        let s = stdout_str(&out);
        assert_eq!(s, *expected, "[{name}] output mismatch");
    }
}
