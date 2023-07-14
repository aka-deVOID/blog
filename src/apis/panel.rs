/// Admin Panel Apis
/// TODO: Fix upload Image, Add safe CRUD for images or maybe add some model
use crate::{
    models::article::{
        ActiveModel as ArticleActiveModel, Column as ArticleColumn, Entity as Article,
        Model as ArticleModel, Status,
    },
    prelude::*,
    serializers::article::{CreateArticle, Image},
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sea_orm::{entity::*, prelude::*, query::*, ActiveValue::Set, DatabaseBackend, EntityTrait};
use serde_json::json;
use slugify::slugify;

/// Done
#[post("/article/create/")]
pub async fn create_article_api(
    content: web::Json<CreateArticle>,
    db: web::Data<AppState>,
) -> ResponseResult {
    let desc = {
        let len = content.content.len();
        if len < 220 {
            content.content[0..len].to_string()
        } else {
            content.content[0..220].to_string()
        }
    };

    ArticleActiveModel {
        title: Set(content.title.clone()),
        slug: Set(slugify!(&content.title, max_length = 440)),
        desc: Set(desc),
        image: Set(content.image),
        content: Set(content.content.clone()),
        status: Set(content.status),
        ..Default::default()
    }
    .save(&db.conn)
    .await?;

    Ok(HttpResponse::Created().json(json!({"ok": "created"})))
}

/// Done
#[get("/article/{id}/")]
pub async fn get_article_by_id_api(
    path: web::Path<i32>,
    db: web::Data<AppState>,
) -> ResponseResult {
    let article = match Article::find_by_id(path.into_inner())
        .into_json()
        .one(&db.conn)
        .await?
    {
        Some(article) => article,
        None => {
            return Ok(HttpResponse::NotFound().finish());
        }
    };

    Ok(HttpResponse::Ok().json(article))
}

/// TODO: save bytes
#[post("/article/img/upload/")]
pub async fn upload_image_api(data: web::Bytes) -> ResponseResult {
    todo!()
}

/// TODO: delete image
#[post("/article/img/{id}/delete")]
pub async fn delete_image_api(path: web::Path<i32>) -> ResponseResult {
    todo!()
}

/// TODO: update article
#[put("/article/{id}/update")]
async fn update_article_api(path: web::Path<i32>) -> impl Responder {
    ""
}

/// Done
#[delete("/article/{id}/")]
pub async fn delete_article_api(path: web::Path<i32>, db: web::Data<AppState>) -> ResponseResult {
    Article::delete_by_id(path.into_inner())
        .exec(&db.conn)
        .await?;
    Ok(HttpResponse::Ok().finish())
}
