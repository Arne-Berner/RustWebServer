use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    // export database url to make it really work.
    // better yet, use an .env file
    cli::run_cli(migration::Migrator).await;
}
