/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20230321_053747_create_author_table;
mod m20230321_054811_create_book_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20230321_053747_create_author_table::Migration),
            Box::new(m20230321_054811_create_book_table::Migration),
        ]
    }
}
