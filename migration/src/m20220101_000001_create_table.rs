use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .create_table(
                Table::create()
                    .table(Paste::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Paste::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Paste::Date).big_integer().not_null())
                    .col(ColumnDef::new(Paste::Uuid).uuid().unique_key().not_null())
                    .col(ColumnDef::new(Paste::Title).string().not_null())
                    .col(ColumnDef::new(Paste::Text).string().not_null())
                    .col(ColumnDef::new(Paste::Deletekey).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .drop_table(Table::drop().table(Paste::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Paste {
    Table,
    Id,
    Date,
    Uuid,
    Title,
    Text,
    Deletekey,
}
