use std::path::Path;
use std::path::PathBuf;

// Clap helpers
use clap::Parser;
use clap::Subcommand;

// File helpers
use directories::BaseDirs;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

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


fn main() -> Result<(), std::io::Error>{
    if let Some(base_dirs) = BaseDirs::new() {
        let dir: &Path = base_dirs.config_dir();
        let mut file = File::create_new(dir.join("hugit-hooks.toml"))?;
        file.write_all(b"allow_repos = []");
    }
    Ok(())
}
