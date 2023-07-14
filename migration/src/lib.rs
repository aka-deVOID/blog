pub use sea_orm_migration::prelude::*;

mod m20230708_204117_article;
mod m20230709_153557_category;
mod m20230709_153603_tag;
mod m20230709_172134_article_category;
mod m20230709_172137_article_tag;
mod m20230714_113351_image;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230708_204117_article::Migration),
            Box::new(m20230709_153557_category::Migration),
            Box::new(m20230709_153603_tag::Migration),
            Box::new(m20230709_172134_article_category::Migration),
            Box::new(m20230709_172137_article_tag::Migration),
            Box::new(m20230714_113351_image::Migration),
        ]
    }
}
