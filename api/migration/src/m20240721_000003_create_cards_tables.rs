use sea_orm_migration::prelude::*;

use crate::m20240721_000001_create_user_table::User;
use crate::m20240721_000004_create_status_tables::StatusType;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240721_000003_create_cards_tables" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CardRanking::Table)
                    .col(
                        ColumnDef::new(CardRanking::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CardRanking::Name).string().not_null())
                    .col(
                        ColumnDef::new(CardRanking::DamageMultiplicator)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CardRanking::CostMultiplicator)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CardType::Table)
                    .col(
                        ColumnDef::new(CardType::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CardType::Name).string().not_null())
                    .col(ColumnDef::new(CardType::StatusTypeId).integer().null())
                    .col(ColumnDef::new(CardType::Damage).integer().null())
                    .col(ColumnDef::new(CardType::Cost).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(StatusType::Id)
                            .to_col(CardType::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Card::Table)
                    .col(
                        ColumnDef::new(Card::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Card::UserId).integer().not_null())
                    .col(ColumnDef::new(Card::CardTypeId).integer().not_null())
                    .col(ColumnDef::new(Card::CardRankingId).integer().not_null())
                    .col(ColumnDef::new(Card::Cost).integer().not_null())
                    .col(
                        ColumnDef::new(Card::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(ForeignKey::create().from_col(User::Id).to_col(Card::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(CardType::Id)
                            .to_col(Card::CardTypeId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(CardRanking::Id)
                            .to_col(Card::CardRankingId),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the User table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Card::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CardType::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(CardRanking::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum CardRanking {
    Table,
    Id,
    Name,
    DamageMultiplicator,
    CostMultiplicator,
}

#[derive(Iden)]
pub enum CardType {
    Table,
    Id,
    Name,
    StatusTypeId,
    Damage,
    Cost,
}

#[derive(Iden)]
pub enum Card {
    Table,
    Id,
    UserId,
    CardTypeId,
    CardRankingId,
    Cost,
    CreatedAt,
}
