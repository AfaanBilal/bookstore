/**
 * 📕 BookStore
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/bookstore
 */
use sea_orm_migration::prelude::*;

use super::m20220101_000001_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Author::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Author::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-author-user_id")
                            .from(Author::Table, Author::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Author::Firstname).string().not_null())
                    .col(ColumnDef::new(Author::Lastname).string().not_null())
                    .col(ColumnDef::new(Author::Bio).string().not_null())
                    .col(
                        ColumnDef::new(Author::CreatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Author::UpdatedAt)
                            .timestamp()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Author::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Author {
    Table,
    Id,
    UserId,
    Firstname,
    Lastname,
    Bio,
    CreatedAt,
    UpdatedAt,
}
