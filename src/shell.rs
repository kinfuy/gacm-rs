use std::process::{Command, Output};
pub fn run(command: &str, args: Vec<&str>) -> Output {
    Command::new(command).args(args).output().unwrap()
}
