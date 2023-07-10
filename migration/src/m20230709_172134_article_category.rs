use crate::{m20230708_204117_article::Article, m20230709_153557_category::Category};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum CategoryArticle {
    Table,
    Id,
    Article,
    Category,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(CategoryArticle::Table)
                    .col(
                        ColumnDef::new(CategoryArticle::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryArticle::Article)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryArticle::Category)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_to_article")
                            .from(CategoryArticle::Table, CategoryArticle::Article)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_to_category")
                            .from(CategoryArticle::Table, CategoryArticle::Category)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CategoryArticle::Table).to_owned())
            .await
    }
}
