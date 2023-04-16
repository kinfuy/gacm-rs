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

#[derive(Debug, Parser, Clone)]
pub struct UseArgs {
    name: String,
    #[clap(short, long)]
    pub local: bool,
    #[clap(short, long)]
    pub global: bool,
    #[clap(short, long)]
    pub system: bool,
}

pub fn get_users() -> Vec<User> {
    let mut _config = GacmConfig::new();
    let config = _config.load().unwrap();
    let _account = config.get_use_config();
    _account.to_vec()
}

pub fn use_account(args: UseArgs) {
    let env = if args.system {
        "system"
    } else if args.global {
        "global"
    } else {
        "local"
    };
    let _account: Vec<User> = get_users();
    let current = _account.iter().find(|user| user.alias == args.name);
    if let Some(cur) = current {
        shell::run("git", vec!["config", "--local", "user.name", &cur.name]);
        shell::run("git", vec!["config", "--local", "user.email", &cur.email]);
        println!("\n");
        logger::success(&format!(
            "git user changed [{}]:{}({})",
            env, cur.alias, cur.name
        ))
    }
}

pub fn ls_account() {
    let account: Vec<User> = get_users();
    let max_len = User::max_size(&account) + 8;
    let current_user = shell::run_str("git", vec!["config", "user.name"]);
    let current_email = shell::run_str("git", vec!["config", "user.email"]);

    println!("\n");
    for user in account.iter() {
        let is_current = if user.name == current_user && user.email == current_email {
            true
        } else {
            false
        };

        let ouput = user.format(max_len, is_current);
        logger::info(&ouput)
    }
}
