use crate::{
    models::{
        article::{
            ActiveModel as ArticleActiveModel, Column as ArticleColumn, Entity as Article,
            Model as ArticleModel, Status,
        },
        category::Entity as Category,
        category_article::{Entity as ArticleCategory, Model as CategoryArticleModel},
    },
    prelude::*,
    serializers::article::{CreateArticle, Image},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sea_orm::{
    entity::*, prelude::*, query::LoaderTrait, query::*, ActiveValue::Set, DatabaseBackend,
    EntityTrait,
};
use serde_json::json;

#[get("/article/{slug}/")]
pub async fn get_article_by_slug_api(
    path: web::Path<String>,
    db: web::Data<AppState>,
) -> ResponseResult {
    let article = Article::find()
        .filter(
            Condition::all()
                .add(ArticleColumn::Slug.eq(path.into_inner()))
                .add(ArticleColumn::Status.eq("published")),
        )
        .one(&db.conn)
        .await?;
    // TODO: load many to many relation with category and tags
    Ok(HttpResponse::Ok().json(json!({ "ok": article })))
}

#[get("/")]
pub async fn get_article_list(db: web::Data<AppState>) -> ResponseResult {
    let articles = Article::find()
        .filter(ArticleColumn::Status.eq("published"))
        .select_only()
        .columns([
            ArticleColumn::Image,
            ArticleColumn::Title,
            ArticleColumn::Slug,
            ArticleColumn::Desc,
            ArticleColumn::CreatedAt,
        ])
        .into_json()
        .all(&db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(json!({ "ok": articles })))
}
