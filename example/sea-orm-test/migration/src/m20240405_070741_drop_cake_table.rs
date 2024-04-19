use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240405_070741_drop_cake_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cake::Table).if_exists().to_owned())
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // the down is necessary even for up statements
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Cake {
    Table,
    Id,
    Name,
}
