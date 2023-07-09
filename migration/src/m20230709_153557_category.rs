use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Category {
    Table,
    Id,
    Title,
    Slug,
    Description,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Category::Title).string_len(120).not_null())
                    .col(ColumnDef::new(Category::Slug).string_len(240).not_null())
                    .col(ColumnDef::new(Category::Description).string_len(300))
                    .col(ColumnDef::new(Category::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Category::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}
