use clap::{Parser, Subcommand};

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

mod commands;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Init {},
    CatFile {
        #[clap(short, action = clap::ArgAction::SetTrue)]
        p: bool,

        object_name: String,
    },
    HashObject {
        #[clap(short, action = clap::ArgAction::SetTrue)]
        w: bool,

        path: String,
    },
    LsTree {
        #[clap(long, action = clap::ArgAction::SetTrue)]
        name_only: bool,

        object_name: String,
    },
}

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
        Commands::CatFile { object_name, .. } => {
            commands::cat_file(&object_name);
        }
        Commands::HashObject { path, .. } => {
            commands::hash_object(&path);
        }
        Commands::LsTree {
            name_only,
            object_name,
        } => {
            commands::ls_tree(name_only, &object_name);
        }
    }
}
