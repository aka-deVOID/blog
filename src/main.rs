mod apis;
mod error;
mod middlewares;
mod models;
mod prelude;
mod response_error;
mod serializers;
mod state;

extern crate slugify;

use crate::apis::article::create_article_api;
use crate::error::Result;
use crate::state::AppState;
use actix_web::{middleware, web, App, HttpServer};
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
            .service(create_article_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    Ok(())
}
