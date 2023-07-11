use crate::prelude::*;
use crate::serializers::article::CreateArticle;
use actix_web::{delete, get, patch, post, put, web, Responder};

#[post("/article/create/")]
pub async fn create_article_api(
    content: web::Json<CreateArticle>,
    db: web::Data<AppState>,
) -> impl Responder {
    
    println!("{content:?}");
    ""
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
