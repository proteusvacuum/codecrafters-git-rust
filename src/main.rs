use clap::{Parser, Subcommand};

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Init {},
    CatFile {
        #[clap(short)]
        p: String,
    },
    HashObject {
        w: String,
    },
}
mod commands;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init {} => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        }
        Commands::CatFile { p } => {
            commands::cat_file::cat_file(&p);
        }
        Commands::HashObject { w } => {
            commands::hash_object::hash_object(&w);
        }
    }
}
