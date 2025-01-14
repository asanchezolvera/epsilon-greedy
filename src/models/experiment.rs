use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Experiment {
  pub id: uuid::Uuid,
  pub name: String,
  pub variants: Vec<Variant>,
  pub epsilon: f64,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
  pub id: uuid::Uuid,
  pub name: String,
  pub value: f64,
  pub count: i64,
}