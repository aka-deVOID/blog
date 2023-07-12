use crate::{
    models::article::{
        ActiveModel as ArticleActiveModel, Column as ArticleColumn, Entity as Article,
        Model as ArticleModel, Status,
    },
    prelude::*,
    serializers::article::CreateArticle,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sea_orm::{entity::*, prelude::*, query::*, ActiveValue::Set, DatabaseBackend, EntityTrait};
use serde_json::json;
use slugify::slugify;

#[post("/admin/article/create/")]
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

    let article = ArticleActiveModel {
        title: Set(content.title.clone()),
        slug: Set(slugify!(&content.title, max_length = 440)),
        desc: Set(desc),
        content: Set(content.content.clone()),
        status: Set(content.status),
        ..Default::default()
    };
    article.save(&db.conn).await?;
    Ok(HttpResponse::Created().json(json!({"ok": "created"})))
}

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
    println!("{:?}", article);

    Ok(HttpResponse::Ok().json(json!({ "ok": article })))
}

#[get("/article/")]
pub async fn get_article_list(db: web::Data<AppState>) -> ResponseResult {
    let articles = Article::find()
        .filter(ArticleColumn::Status.eq("published"))
        .into_json()
        .all(&db.conn)
        .await?;
    Ok(HttpResponse::Ok().json(json!({ "ok": articles })))
}

#[get("/admin/article/{id}/")]
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

#[post("/article/img/upload/")]
async fn upload_article_image_api() -> impl Responder {
    ""
}

#[put("/article/{id}/")]
async fn update_article_api(path: web::Path<i32>) -> impl Responder {
    ""
}

#[delete("/article/{id}/")]
pub async fn delete_article_api(path: web::Path<i32>, db: web::Data<AppState>) -> ResponseResult {
    Article::delete_by_id(path.into_inner())
        .exec(&db.conn)
        .await?;
    Ok(HttpResponse::Ok().finish())
}
