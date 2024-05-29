use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    sitebasicinfo: SiteBasicInfo,
}


#[derive(Clone, Deserialize, Serialize)]
struct SiteBasicInfo {
    title: String,
    subtitle: String,
    description: String,
    author: String,
    favicon: String,
    avatar: String,
}


impl Default for Config {
    fn default() -> Self {
        let config_path = "./config.json";
        match fs::read_to_string(config_path) {
            Ok(config_content) => {
                serde_json::from_str(&config_content).expect("config.json not ready")
            },
            Err(_) => {
                println!("not found config.json");
                Config {
                    sitebasicinfo: SiteBasicInfo::default(),
                }
            }
        }
    }
}

impl Default for SiteBasicInfo {
    fn default() -> Self {
        SiteBasicInfo {
            title: String::default(),
            subtitle: String::default(),
            description: String::default(),
            author: String::default(),
            favicon: String::default(),
            avatar: String::default(),
        }
    }
}