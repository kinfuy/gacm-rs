mod config;

use clap::{arg, Command};
use std::path::PathBuf;

pub fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}

pub fn cli() -> Command {
    Command::new("gacm")
        .about("Git Account Management Tools & NPM Registry Manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("ls")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("diff")
                .about("Compare two commits")
                .arg(arg!(base: [COMMIT]))
                .arg(arg!(head: [COMMIT]))
                .arg(arg!(path: [PATH]).last(true))
                .arg(
                    arg!(--color <WHEN>)
                        .value_parser(["always", "auto", "never"])
                        .num_args(0..=1)
                        .require_equals(true)
                        .default_value("auto")
                        .default_missing_value("always"),
                ),
        )
        .subcommand(
            Command::new("push")
                .about("pushes things")
                .arg(arg!(<REMOTE> "The remote to target"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("adds things")
                .arg_required_else_help(true)
                .arg(arg!(<PATH> ... "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("stash")
                .args_conflicts_with_subcommands(true)
                .args(push_args())
                .subcommand(Command::new("push").args(push_args()))
                .subcommand(Command::new("pop").arg(arg!([STASH])))
                .subcommand(Command::new("apply").arg(arg!([STASH]))),
        )
}
