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
    WriteTree {},
    CommitTree {
        tree_sha: String,

        #[clap(short)]
        p: String,

        #[clap(short)]
        m: String,
    },
    Clone {
        repo_url: String,
        dir: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init {} => {
            commands::init();
        }
        Commands::CatFile { object_name, .. } => {
            print!("{}", commands::cat_file(object_name));
        }
        Commands::HashObject { path, .. } => {
            print!("{}", commands::hash_object(path));
        }
        Commands::LsTree {
            name_only,
            object_name,
        } => {
            print!("{}", commands::ls_tree(name_only, object_name));
        }
        Commands::WriteTree {} => {
            print!("{}", commands::write_tree());
        }
        Commands::CommitTree {
            tree_sha,
            p: parent_sha,
            m: message,
        } => {
            print!("{}", commands::commit_tree(tree_sha, parent_sha, message));
        }
        Commands::Clone { repo_url, dir } => {
            commands::clone(repo_url, dir);
        }
    }
}
