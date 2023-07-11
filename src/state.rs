use crate::error::Result;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

impl AppState {
    pub async fn close(self) -> Result<()> {
        self.conn.close().await?;
        Ok(())
    }
}

impl std::ops::Deref for AppState {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}
