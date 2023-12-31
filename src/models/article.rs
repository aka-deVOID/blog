//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Status {
    #[sea_orm(string_value = "published")]
    #[serde(rename = "published")]
    Published,
    #[sea_orm(string_value = "draft")]
    #[serde(rename = "draft")]
    Draft,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "article")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub image: Option<i32>,
    pub content: String,
    pub desc: String,
    pub status: Status,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::category_article::Entity")]
    CategoryArticle,
    #[sea_orm(
        belongs_to = "super::image::Entity",
        from = "Column::Image",
        to = "super::image::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Image,
    #[sea_orm(has_many = "super::tag_article::Entity")]
    TagArticle,
}

impl Related<super::category_article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CategoryArticle.def()
    }
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl Related<super::tag_article::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TagArticle.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
