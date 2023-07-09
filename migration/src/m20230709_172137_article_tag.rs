pub(crate) use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum TagArticle {
    Table,
    Articles,
    Categories,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TagArticle::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TagArticle::Articles).integer().not_null())
                    .col(ColumnDef::new(TagArticle::Categories).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
