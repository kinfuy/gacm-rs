use colorized::Colors;
use dirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

use unicode_segmentation::UnicodeSegmentation;
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub alias: String,
}
impl User {
    pub fn format(&self, len: usize, is_current: bool) -> String {
        let start_len = User::len(&self.alias) + User::len(&self.name);
        let end_len = User::len(&self.email);
        let len = len - start_len - end_len;
        let current = if is_current {
            colorized::colorize_this("■", Colors::GreenFg)
        } else {
            String::from(" ")
        };
        format!(
            "  {} {}({}){}{}",
            current,
            self.alias,
            colorized::colorize_this(&self.name, Colors::WhiteFg),
            String::from("-").repeat(len),
            self.email
        )
    }
    pub fn len(str: &str) -> usize {
        let graphemes = UnicodeSegmentation::graphemes(str, true).collect::<Vec<&str>>();
        graphemes.len()
    }
    pub fn max_size(users: &Vec<User>) -> usize {
        let mut max = 0;
        for itme in users.iter() {
            let length = User::len(&itme.name) + User::len(&itme.alias) + User::len(&itme.email);
            max = if max < length { length } else { max }
        }
        max
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Registry {
    name: String,
    alias: String,
    home: String,
    registry: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GacmConfig {
    pub version: String,
    pub users: Vec<User>,
    pub registry: Vec<Registry>,
    pub is_load: Option<bool>,
}

impl GacmConfig {
    pub fn new() -> GacmConfig {
        GacmConfig {
            version: String::from("0.0.1"),
            registry: Vec::new(),
            users: Vec::new(),
            is_load: Some(false),
        }
    }
    pub fn load(&mut self) -> Result<&GacmConfig, Box<dyn Error>> {
        match self.is_load {
            Some(false) => {
                let home_dir = dirs::home_dir().unwrap();
                let config_str = fs::read_to_string(home_dir.join(".gacmrc"))?;
                let config: GacmConfig = serde_json::from_str(&config_str)?;
                self.users = config.users;
                self.registry = config.registry;
                self.version = config.version;
                self.is_load = Some(true);
                Ok(self)
            }
            _ => Ok(self),
        }
    }

    pub fn get_use_config(&self) -> &Vec<User> {
        &self.users
    }
}
