use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Err>;

#[derive(Debug)]
pub enum Err {
    Io(std::io::Error),
    SeaOrm(sea_orm::error::DbErr),
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Err::Io(..) => write!(f, "Io Error"),
            Err::SeaOrm(_) => write!(f, "SeaOrm Error"),
        }
    }
}

impl From<std::io::Error> for Err {
    fn from(value: std::io::Error) -> Self {
        Err::Io(value)
    }
}

impl From<sea_orm::error::DbErr> for Err {
    fn from(value: sea_orm::error::DbErr) -> Self {
        Err::SeaOrm(value)
    }
}

impl std::error::Error for Err {}
