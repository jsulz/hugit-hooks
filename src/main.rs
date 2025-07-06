use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

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


fn main() {
    let args = CLI::parse();
}
