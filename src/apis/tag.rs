use crate::models::tag::{
    ActiveModel as TagActiveModel, Column as TagColumn, Entity as Tag, Model as TagModel,
};
use crate::prelude::*;
use crate::serializers::category::CreateCategory;
use crate::serializers::tag::CreateTag;
use actix_web::{delete, get, post, web, HttpResponse};
use sea_orm::{entity::*, prelude::*, query::*, ActiveValue::Set, DatabaseBackend, EntityTrait};
use serde_json::json;
use slugify::slugify;

#[post("/tag/create/")]
pub async fn create_tag_api(data: web::Json<CreateTag>, db: web::Data<AppState>) -> ResponseResult {
    TagActiveModel {
        title: Set(data.title.clone()),
        slug: Set(slugify!(&data.title)),
        ..Default::default()
    }
    .save(&db.conn)
    .await?;

    Ok(HttpResponse::Created().finish())
}

#[delete("/tag/{id}/")]
pub async fn delete_tag_api(path: web::Path<i32>, db: web::Data<AppState>) -> ResponseResult {
    Tag::delete_by_id(path.into_inner()).exec(&db.conn).await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/tag/list/")]
pub async fn get_list_tag_api(db: web::Data<AppState>) -> ResponseResult {
    let tags = Tag::find()
        .select_only()
        .columns([TagColumn::Title, TagColumn::Slug])
        .into_json()
        .all(&db.conn)
        .await?;

    Ok(HttpResponse::Ok().json(json!({ "ok": tags })))
}
