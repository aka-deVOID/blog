mod apis;
mod error;
mod middlewares;
mod models;
mod response_error;
mod serializers;

use crate::error::Result;
use actix_web::{middleware, web, App, HttpServer};
use dotenvy;
use sea_orm::{Database, DatabaseConnection};

#[derive(Copy)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

impl std::ops::Deref for AppState {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let db: DatabaseConnection = Database::connect("sqlite://test.db").await?;
    let state = web::Data::new(AppState { conn: db });

    HttpServer::new(|| {
        App::new()
            .app_data(state)
            .wrap(middleware::Compress::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    Ok(())
}
