use migration::Migrator;

pub async fn init() {
    migration::cli::run_cli(Migrator).await
}
