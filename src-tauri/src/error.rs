use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct AppError {
    pub source: String,
    pub reason: String,
}
