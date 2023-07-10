use crate::m20230708_204117_blog::Article;
use crate::m20230709_153603_tag::Tag;
pub(crate) use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum TagArticle {
    Table,
    Id,
    Article,
    Tag,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TagArticle::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TagArticle::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TagArticle::Article).integer().not_null())
                    .col(ColumnDef::new(TagArticle::Tag).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_to_article")
                            .from(TagArticle::Table, TagArticle::Article)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_to_tag")
                            .from(TagArticle::Table, TagArticle::Tag)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TagArticle::Table).to_owned())
            .await
    }
}
