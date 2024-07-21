use sea_orm_migration::prelude::*;

use crate::m20240721_000001_create_user_table::User;
use crate::m20240721_000004_create_status_tables::StatusType;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240721_000006_create_items_tables" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ItemType::Table)
                    .col(
                        ColumnDef::new(ItemType::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ItemType::Name).string().not_null())
                    .col(ColumnDef::new(ItemType::StatusTypeId).integer().not_null())
                    .col(ColumnDef::new(ItemType::Damage).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(StatusType::Id)
                            .to_col(ItemType::StatusTypeId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .col(
                        ColumnDef::new(Item::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Item::UserId).string().not_null())
                    .col(ColumnDef::new(Item::ItemTypeId).integer().not_null())
                    .col(ColumnDef::new(Item::Multiplicator).integer().not_null())
                    .col(
                        ColumnDef::new(Item::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create().from_col(User::Id).to_col(Item::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(ItemType::Id)
                            .to_col(Item::ItemTypeId),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ItemType::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ItemType {
    Table,
    Id,
    Name,
    StatusTypeId,
    Damage,
}

#[derive(Iden)]
pub enum Item {
    Table,
    Id,
    UserId,
    ItemTypeId,
    Multiplicator,
    CreatedAt,
}
