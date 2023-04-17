use colorized::Colors;
use futures::join;
use url::Url;

use clap::{Parser, Subcommand};

use crate::{
    config::{GacmConfig, Registry},
    logger, shell,
};

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

pub async fn query_registry(command: &str) -> String {
    shell::run_str(command, vec!["config", "get", "registry"])
}

pub fn use_registry(_args: UseArgs) {}

#[derive(Debug, Clone)]
pub struct RegistryItem {
    pub tag: String,
    pub name: String,
    pub registry: String,
}

#[derive(Clone)]
pub struct RegistryManager {
    list: Vec<RegistryItem>,
}
impl RegistryManager {
    pub fn new(list: Vec<RegistryItem>) -> RegistryManager {
        RegistryManager { list }
    }
    pub fn get(&self, registry: &str) -> String {
        let mut display = String::new();
        for item in self.list.iter() {
            if !item.registry.is_empty() {
                let current_url = Url::parse(&item.registry).unwrap();
                let match_registry = Url::parse(&registry).unwrap();
                if current_url.as_str() == match_registry.as_str() {
                    display.push_str(" ");
                    display.push_str(&format!("{}", &item.tag))
                }
            }
        }
        display
    }
}

pub async fn ls_registry() {
    let registry = get_registry();
    let npm_registry = query_registry("npm");
    let yarn_registry = query_registry("yarn");
    let cnpm_registry = query_registry("cnpm");
    let (npm_registry, yarn_registry, cnpm_registry) =
        join!(npm_registry, yarn_registry, cnpm_registry);
    let max_len = Registry::max_size(&registry) + 2;

    let npm: RegistryItem = RegistryItem {
        name: "npm".to_string(),
        registry: npm_registry.to_owned(),
        tag: colorized::colorize_this("■", Colors::GreenFg),
    };
    let pnpm: RegistryItem = RegistryItem {
        name: "pnpm".to_string(),
        registry: npm_registry.to_owned(),
        tag: colorized::colorize_this("■", Colors::YellowFg),
    };
    let yarn: RegistryItem = RegistryItem {
        name: "yarn".to_string(),
        registry: yarn_registry,
        tag: colorized::colorize_this("■", Colors::BlueFg),
    };
    let cnpm: RegistryItem = RegistryItem {
        name: "cnpm".to_string(),
        registry: cnpm_registry,
        tag: colorized::colorize_this("■", Colors::RedFg),
    };
    let pkg_mangager = RegistryManager::new(vec![npm, yarn, pnpm, cnpm]);

    println!("\n");
    let mut tips = String::from("current: ");
    for item in pkg_mangager.list.iter() {
        if !item.registry.is_empty() {
            tips.push_str(&format!("{} {} ", item.name, item.tag))
        }
    }
    println!("{}", tips);
    println!("\n");
    for registry in registry.iter() {
        let ouput = registry.format(max_len, &pkg_mangager);
        logger::info(&ouput)
    }
}
