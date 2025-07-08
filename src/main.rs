// Path helpers
use std::path::Path;
use std::path::PathBuf;

// Clap helpers
use clap::Parser;
use clap::Subcommand;

// File helpers
use directories::ProjectDirs;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;
use std::io::prelude::*;

use serde::Serialize;
use serde::Deserialize;

// Implement with subcommand since this is nested under the top-level enum
#[derive(Debug, Subcommand)]
#[derive(Clone)]
enum Commands {
    Allow {
        path: PathBuf
    },
    List,
    Install,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct CLI {
    // this field is a subcommand
    #[command(subcommand)] 
    command: Commands
}

#[derive(Deserialize, Serialize)]
struct Config {
    allowed_repos: Vec<PathBuf>
}

impl Config {
    fn load(&self) -> Result<Config, Box<dyn std::error::Error>>{
        let dir: ProjectDirs = ProjectDirs::from("com", "jsulz", "hugit-hooks")
            .ok_or_else(|| anyhow::anyhow!("could not determine config directory"))?;
        let config_path: PathBuf = dir.config_dir().join("config.toml");

        match read_to_string(&config_path) {
            Ok(conf_contents) => {
                // Try to read from the toml file into the a config struct and return
                match toml::from_str::<Config>(&conf_contents) {
                    Ok(config) => return Ok(config),
                    Err(e) => return Ok(Config{ allowed_repos: vec![PathBuf::new()]})

                }
            }
            // An error here means this file doesn't exist and we should just return conf
            Err(e) => return Ok(Config{ allowed_repos: vec![PathBuf::new()]})
        }
    }
    
    fn save(&self){

    }

    fn is_allowed(&self, repo_path: PathBuf){

    }

    fn add_repo(&self, repo_path: PathBuf){

    }
}


fn main() {
    let conf = Config{ allowed_repos: vec![PathBuf::new()]};
    conf.load();
}
