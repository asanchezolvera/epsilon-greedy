// main.rs
use axum::{
  routing::{get, post},
  Router,
};
use sqlx::PgPool;
use std::sync::Arc;

mod api;
mod config;
mod error;
mod models;
mod services;

#[tokio::main]
async fn main() {
  // Initialize configuration
  let config = config::load_config().expect("Failed to load configuration");
  
  // Set up database connection pool
  let pool = PgPool::connect(&config.database_url)
      .await
      .expect("Failed to connect to database");
  
  // Initialize redis connection
  let redis_client = redis::Client::open(config.redis_url)
      .expect("Failed to connect to Redis");
  
  // Initialize the application state
  let state = Arc::new(AppState {
      db: pool,
      redis: redis_client,
      config: config,
  });
  
  // Build the router
  let app = Router::new()
      .route("/api/experiments/:id/variant", get(api::handlers::get_variant))
      .route("/api/experiments/:id/reward", post(api::handlers::record_reward))
      .route("/api/experiments/:id/stats", get(api::handlers::get_stats))
      .with_state(state);
  
  // Start the server
  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await
      .unwrap();
}