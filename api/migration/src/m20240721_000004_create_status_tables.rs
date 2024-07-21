use sea_orm_migration::prelude::*;

use crate::m20240721_000002_create_tamago_table::Tamago;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240721_000004_create_status_tables" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StatusType::Table)
                    .col(
                        ColumnDef::new(StatusType::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StatusType::Name).string().not_null())
                    .col(ColumnDef::new(StatusType::Damage).integer().not_null())
                    .col(ColumnDef::new(StatusType::Duration).integer().not_null())
                    .col(
                        ColumnDef::new(StatusType::IntervalMinute)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Status::Table)
                    .col(
                        ColumnDef::new(Status::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Status::TamagoId).integer().not_null())
                    .col(ColumnDef::new(Status::StatusTypeId).integer().not_null())
                    .col(ColumnDef::new(Status::Damage).integer().not_null())
                    .col(
                        ColumnDef::new(Status::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(Tamago::Id)
                            .to_col(Status::TamagoId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(StatusType::Id)
                            .to_col(Status::StatusTypeId),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Status::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(StatusType::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum StatusType {
    Table,
    Id,
    Name,
    Damage,
    Duration,
    IntervalMinute,
}

#[derive(Iden)]
pub enum Status {
    Table,
    Id,
    TamagoId,
    StatusTypeId,
    Damage,
    CreatedAt,
}
