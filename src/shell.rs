use std::process::{Command, Output};
use std::str;

pub fn run(command: &str, args: Vec<&str>) -> Output {
    Command::new(command).args(args).output().unwrap()
}

pub fn run_str(command: &str, args: Vec<&str>) -> String {
    let output = Command::new(command).args(args).output();

    if let Ok(out) = output {
        str::from_utf8(&out.stdout)
            .unwrap()
            .to_string()
            .replace("\n", "")
    } else {
        "".to_string()
    }
}
