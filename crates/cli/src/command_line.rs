use crate::commands;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// dlasdlasldas
///
/// dasdasd asdasdasda s
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(subcommand_required = false)]
#[clap(arg_required_else_help = false)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the application
    ///
    /// Create related directories and configuration files used by application
    Init {
        /// dlasdlasldas
        #[clap(short, long)]
        force: bool,
    },

    /// Import bookmarks from other services
    ///
    /// Populate the local database with links from other bookmark services
    Import {
        #[clap(default_value_t = String::from("getpocket"), short, long)]
        provider: String,

        file_path: Option<PathBuf>,
    },

    /// Process a url
    ///
    /// Process a url identified by a informed id
    Process { url_identifier: i32 },

    /// List all bookmarks
    ///
    /// List all saved bookmarks
    List,
}

pub async fn get_cli() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Import {
            file_path,
            provider,
        } => {
            println!("{:#?}", provider);
            commands::import::init(file_path).await;
        }
        Commands::Process { url_identifier } => {
            commands::process::init(url_identifier).await;
        }
        Commands::Init { force } => {
            commands::init::init(force).await;
        }
        Commands::List => {
            commands::list::init().await;
        }
    }
}
