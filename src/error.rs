#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Database error: {0}")]
  Database(#[from] sqlx::Error),
  
  #[error("Cache error: {0}")]
  Cache(#[from] redis::RedisError),
  
  #[error("Experiment not found: {0}")]
  NotFound(uuid::Uuid),
  
  #[error("Invalid configuration: {0}")]
  Configuration(String),
}