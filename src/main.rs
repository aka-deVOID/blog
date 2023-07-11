mod apis;
mod error;
mod middlewares;
mod models;
mod prelude;
mod response_error;
mod serializers;
mod state;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveActiveEnum, EnumIter)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Status {
    #[sea_orm(string_value = "published")]
    #[serde(rename = "published")]
    Published,
    #[sea_orm(string_value = "draft")]
    #[serde(rename = "draft")]
    Draft,
}

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
