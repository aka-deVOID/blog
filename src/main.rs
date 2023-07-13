mod apis;
mod error;
mod middlewares;
mod models;
mod prelude;
mod response_error;
mod serializers;
mod state;

extern crate slugify;

use crate::{
    apis::article::{get_article_by_slug_api, get_article_list},
    error::Result,
    state::AppState,
};
use actix_web::{
    guard,
    middleware::{self, Logger},
    web, App, HttpServer,
};

use crate::apis::panel::{create_article_api, get_article_by_id_api, upload_article_image_api};
use dotenvy;
use env_logger::Env;
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let db: DatabaseConnection = Database::connect("sqlite://test.db").await?;
    let state = web::Data::new(AppState { conn: db });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/admin")
                    .guard(guard::Header("content-type", "application/json"))
                    .service(upload_article_image_api)
                    .service(get_article_by_id_api)
                    .service(create_article_api),
            )
            .service(get_article_by_slug_api)
            .service(get_article_list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
