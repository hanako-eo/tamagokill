pub use sea_orm_migration::prelude::*;

mod m20240721_000001_create_user_table;
mod m20240721_000002_create_tamago_table;
mod m20240721_000003_create_cards_tables;
mod m20240721_000004_create_status_tables;
mod m20240721_000005_create_death_type_table;
mod m20240721_000006_create_items_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240721_000001_create_user_table::Migration),
            Box::new(m20240721_000005_create_death_type_table::Migration),
            Box::new(m20240721_000002_create_tamago_table::Migration),
            Box::new(m20240721_000004_create_status_tables::Migration),
            Box::new(m20240721_000003_create_cards_tables::Migration),
            Box::new(m20240721_000006_create_items_tables::Migration),
        ]
    }
}
