use std::process;

pub fn err(tool: &str, msg: &str) -> ! {
    eprintln!("{tool}: {msg}");
    process::exit(1);
}

pub fn warn(tool: &str, msg: &str) {
    eprintln!("{tool}: {msg}");
}
