use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[clap(parse(try_from_str), default_value_t = false, short, long)]
        force: bool,
    },
    Import {
        #[clap(default_value_t = String::from("getpocket"), short, long)]
        provider: String,
        file_path: Option<PathBuf>,
    },
}

pub async fn get_cli() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Import {
            file_path,
            provider,
        } => {
            println!("{:#?}", provider);
            commands::import::init(file_path).await;
        }
        Commands::Init { force } => {
            println!("{:#?}", force);
            commands::init::init().await;
        }
    }
}
