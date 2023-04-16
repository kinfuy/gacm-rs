use crate::{
    config::{GacmConfig, User},
    logger, shell,
};
use clap::{Parser, Subcommand};
use std::str;

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    Use(UseArgs),
    Ls(LsArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct LsArgs {}

#[derive(Parser, Debug, Clone)]
pub struct UseArgs {
    name: String,
    #[clap(short, long)]
    pub package: Option<String>,
}

pub fn use_account(args: UseArgs) {
    println!("{:?}", args)
}

pub fn ls_account() {
    let mut _config = GacmConfig::new();
    let config = _config.load().unwrap();
    let _account = config.get_use_config();
    let max_len = User::max_size(_account) + 8;
    let user_output = shell::run("git", vec!["config", "user.name"]);
    let email_output = shell::run("git", vec!["config", "user.email"]);
    let current_email = str::from_utf8(&email_output.stdout)
        .unwrap()
        .replace("\n", "");
    let current_user = str::from_utf8(&user_output.stdout)
        .unwrap()
        .replace("\n", "");
    for user in _account.iter() {
        let is_current = if user.name == current_user && user.email == current_email {
            true
        } else {
            false
        };

        let ouput = user.format(max_len, is_current);
        logger::info(&ouput)
    }
}
