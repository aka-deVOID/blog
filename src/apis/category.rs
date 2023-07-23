use crate::models::category::{
    ActiveModel as CategoryActiveModel, Column as CategoryColumn, Entity as Category,
    Model as CategoryModel,
};
use crate::prelude::*;
use crate::serializers::category::CreateCategory;
use actix_web::{delete, get, patch, post, put, web, HttpResponse};
use sea_orm::{entity::*, prelude::*, query::*, ActiveValue::Set, DatabaseBackend, EntityTrait};
use serde_json::json;
use slugify::slugify;

#[get("/category/list/")]
pub async fn get_list_categories_api(db: web::Data<AppState>) -> ResponseResult {
    let categories = Category::find().into_json().all(&db.conn).await?;

    Ok(HttpResponse::Ok().json(json!({ "ok": categories })))
}

#[delete("/category/{id}/")]
pub async fn delete_category_api(path: web::Path<i32>, db: web::Data<AppState>) -> ResponseResult {
    Category::delete_by_id(path.into_inner())
        .exec(&db.conn)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[put("/category/{id}/")]
pub async fn update_category_api() -> ResponseResult {
    todo!()
}

#[post("/category/create/")]
pub async fn create_category_api(
    data: web::Json<CreateCategory>,
    db: web::Data<AppState>,
) -> ResponseResult {
    println!("{:?}", data);
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
