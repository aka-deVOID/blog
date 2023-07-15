use crate::models::category::{
    ActiveModel as CategoryActiveModel, Column as CategoryColumn, Entity as Category,
    Model as CategoryModel,
};
use crate::prelude::*;
use crate::serializers::category::CreateCategory;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::{entity::*, prelude::*, query::*, ActiveValue::Set, DatabaseBackend, EntityTrait};
use serde_json::json;
use slugify::slugify;

#[get("/category/list/")]
pub async fn get_list_categories(db: web::Data<AppState>) -> ResponseResult {
    let categories = Category::find()
        .select_only()
        .columns([
            CategoryColumn::Title,
            CategoryColumn::Slug,
            CategoryColumn::Description,
        ])
        .into_json()
        .all(&db.conn)
        .await?;
    Ok(HttpResponse::Ok().json(json!({ "ok": categories })))
}

#[post("/category/create/")]
pub async fn create_category(
    data: web::Json<CreateCategory>,
    db: web::Data<AppState>,
) -> ResponseResult {
    CategoryActiveModel {
        title: Set(data.title.clone()),
        slug: Set(slugify!(&data.title)),
        description: Set(data.description.clone()),
        ..Default::default()
    }
    .save(&db.conn)
    .await?;

    Ok(HttpResponse::Created().finish())
}
