mod helpers;

use helpers::*;
use std::fs;

#[test]
fn help_version() {
    check_help_version("which");
}

#[test]
fn errors() {
    let cases: &[(&str, &[&str], &str)] = &[
        ("not found", &["nonexistent_command_xyz_123"], "not found"),
        ("no arguments", &[], "which:"),
        ("too many arguments", &["a", "b"], "which:"),
    ];
    for (_name, a, stderr_substr) in cases {
        let out = run("which", a);
        assert_exit_code(&out, 1);
        assert_stderr_contains(&out, stderr_substr);
    }
}

#[test]
fn path_lookup() {
    let dir = tempfile::tempdir().unwrap();
    let bin_name = if cfg!(windows) {
        "mytool.exe"
    } else {
        "mytool"
    };
    let bin = dir.path().join(bin_name);
    fs::write(&bin, "placeholder").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let mut cmd = std::process::Command::new(bin_path("which"));
    cmd.arg("mytool").env("PATH", dir.path());
    if cfg!(windows) {
        cmd.env("PATHEXT", ".exe");
    }
    let out = cmd.output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("mytool"));
}

#[test]
fn all_flag() {
    let dir1 = tempfile::tempdir().unwrap();
    let dir2 = tempfile::tempdir().unwrap();
    let bin_name = if cfg!(windows) {
        "mytool.exe"
    } else {
        "mytool"
    };
    for d in [&dir1, &dir2] {
        let bin = d.path().join(bin_name);
        fs::write(&bin, "placeholder").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }

    let sep = if cfg!(windows) { ";" } else { ":" };
    let path_val = format!("{}{sep}{}", dir1.path().display(), dir2.path().display());
    let mut cmd = std::process::Command::new(bin_path("which"));
    cmd.args(["-a", "mytool"]).env("PATH", &path_val);
    if cfg!(windows) {
        cmd.env("PATHEXT", ".exe");
    }
    let out = cmd.output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert_eq!(s.lines().count(), 2, "expected 2 matches, got: {s:?}");
}
