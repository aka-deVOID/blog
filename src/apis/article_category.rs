use actix_web::{get, post, web, HttpResponse};

use crate::{
    models::category_article::{
        ActiveModel as CategoryArticleActiveModel, Column as CategoryArticleColumn,
        Entity as CategoryArticle, Model as CategoryArticleModel,
    },
    prelude::ResponseResult,
    serializers::category::CreateArticleCategory,
    state::AppState,
};
use sea_orm::{entity::*, prelude::*, query::*, ActiveValue::Set, DatabaseBackend, EntityTrait};

/// Done
#[post("/article/category/create/")]
pub async fn category_tag_rel(
    db: web::Data<AppState>,
    content: web::Json<CreateArticleCategory>,
) -> ResponseResult {
    CategoryArticleActiveModel {
        article: Set(content.article_id),
        category: Set(content.category_id),
        ..Default::default()
    }
    .save(&db.conn)
    .await?;
    Ok(HttpResponse::Created().finish())
}
