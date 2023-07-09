use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum CategoryArticle {
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
                    .if_not_exists()
                    .table(CategoryArticle::Table)
                    .col(
                        ColumnDef::new(CategoryArticle::Articles)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryArticle::Categories)
                            .integer()
                            .not_null(),
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
