use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Tag {
    Table,
    Id,
    Title,
    Slug,
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
                    .table(Tag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tag::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tag::Title).string_len(120).not_null())
                    .col(ColumnDef::new(Tag::Slug).string_len(240).not_null())
                    .col(ColumnDef::new(Tag::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Tag::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await
    }
}
