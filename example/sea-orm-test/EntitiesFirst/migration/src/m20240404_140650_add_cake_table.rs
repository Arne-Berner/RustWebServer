use entities::sea_orm::{sea_query::*, Schema};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        manager
            .create_table(
                schema
                    .create_table_from_entity(entities::cake::Entity)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(
                Table::drop()
                    .table(entities::cake::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
