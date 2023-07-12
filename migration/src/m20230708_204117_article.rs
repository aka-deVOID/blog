use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Article {
    Table,
    Id,
    Title,
    Slug,
    Image,
    Desc,
    Content,
    Status,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}

/// TODO: on postgresql i need to create data type for status

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Article::Id)
                            .integer()
                            .auto_increment()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Article::Title).string_len(220).not_null())
                    .col(
                        ColumnDef::new(Article::Slug)
                            .string_len(440)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Article::Image).string_len(500))
                    .col(ColumnDef::new(Article::Content).text().not_null())
                    .col(ColumnDef::new(Article::Desc).string_len(220).not_null())
                    .col(
                        ColumnDef::new(Article::Status)
                            .string_len(10)
                            .not_null()
                            .default("draft"),
                    )
                    .col(
                        ColumnDef::new(Article::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Article::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}
