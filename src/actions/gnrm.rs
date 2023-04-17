use clap::{Parser, Subcommand};

use crate::{
    config::{GacmConfig, Registry},
    logger, shell,
};

const PACKAGE_MANAGER: [&str; 4] = ["npm", "yarn", "cnpm", "pnpm"];

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

pub fn get_registry() -> Vec<Registry> {
    let mut _config = GacmConfig::new();
    let config = _config.load().unwrap();
    let registry = config.get_registry_config();
    registry.to_vec()
}

pub fn use_registry(args: UseArgs) {}

pub fn ls_registry() {
    let registry = get_registry();
    let max_len = Registry::max_size(&registry) + 8;
}
