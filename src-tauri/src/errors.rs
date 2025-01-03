use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("The item with a specified id is not found")]
    RowNotFound,
    #[error(transparent)]
    Unexpected(anyhow::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(val: anyhow::Error) -> Self {
        Self::Unexpected(val)
    }
}

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        Self::Unexpected(anyhow::anyhow!(value))
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
