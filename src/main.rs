mod apis;
mod error;
mod middlewares;
mod models;
mod prelude;
mod response_error;
mod serializers;
mod state;

extern crate slugify;

use crate::apis::article::get_article_by_id_api;
use crate::{
    apis::article::{create_article_api, get_article_by_slug_api},
    error::Result,
    state::AppState,
};
use actix_web::{guard, middleware, web, App, HttpServer};
use apis::article::{delete_article_api, get_article_list};
use dotenvy;
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let db: DatabaseConnection = Database::connect("sqlite://test.db").await?;
    let state = web::Data::new(AppState { conn: db });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Compress::default())
            .service(
                web::resource("/admin")
                    .name("admin")
                    .guard(guard::Header("content-type", "application/json")),
            )
            .service(create_article_api)
            .service(get_article_by_slug_api)
            .service(get_article_by_id_api)
            .service(get_article_list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
