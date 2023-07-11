use crate::models::article::ActiveModel as ArticleModel;
use crate::prelude::*;
use crate::serializers::article::CreateArticle;
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use serde_json::json;
use slugify::slugify;

#[post("/article/create/")]
pub async fn create_article_api(
    content: web::Json<CreateArticle>,
    db: web::Data<AppState>,
) -> ResponseResult {
    let article = ArticleModel {
        title: Set(content.title.clone()),
        slug: Set(slugify!(&content.title, max_length = 440)),
        content: Set(content.content.clone()),
        status: Set(content.status),
        ..Default::default()
    };
    article.insert(&db.conn).await?;
    Ok(HttpResponse::Ok().json(json!({"ok": "created"})))
}

#[get("/article/<slug>/")]
async fn get_article_api(path: web::Path<String>) -> impl Responder {
    ""
}

#[post("/article/img/upload/")]
async fn upload_article_image_api() -> impl Responder {
    ""
}

#[put("/article/<id>/")]
async fn update_article_api(path: web::Path<i32>) -> impl Responder {
    ""
}

#[delete("/article/<id>/")]
async fn delete_article_api(path: web::Path<i32>) -> impl Responder {
    ""
}
