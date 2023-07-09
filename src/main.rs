mod error;
mod models;

use crate::error::Result;
use actix_web::{App, HttpServer};
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> Result<()> {
    let db: DatabaseConnection = Database::connect("sqlite://test.db").await?;
    
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    Ok(())
}
