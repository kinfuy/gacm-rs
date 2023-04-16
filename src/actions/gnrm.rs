use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct LsArgs {}

#[derive(Parser, Debug, Clone)]
pub struct UseArgs {
    name: String,
    #[clap(short, long)]
    pub package: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    Use(UseArgs),
    Ls(LsArgs),
}

pub fn use_registry(args: UseArgs) {}
pub fn ls_registry() {}
