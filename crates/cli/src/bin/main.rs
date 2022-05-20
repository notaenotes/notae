use anyhow::Result;
use cli::command_line;

#[async_std::main]
async fn main() -> Result<()> {
    command_line::get_cli().await;
    // data::list_all().await;
    Ok(())
}
