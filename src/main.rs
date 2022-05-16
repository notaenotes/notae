mod cli;
mod commands;
mod data;
mod database;
mod import;
mod settings;

use anyhow::Result;
// use data::*;

#[async_std::main]
async fn main() -> Result<()> {
    cli::get_cli().await;
    // import().await;
    // list_all().await;
    Ok(())
}
