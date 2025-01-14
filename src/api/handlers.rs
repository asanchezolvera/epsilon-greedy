pub async fn get_variant(
  State(state): State<Arc<AppState>>,
  Path(experiment_id): Path<uuid::Uuid>,
  session_id: Option<String>,
) -> Result<Json<Variant>, Error> {
  let session_id = session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
  let bandit = BanditService::new(state.redis.clone(), state.db.clone());
  let variant = bandit.get_variant(experiment_id, &session_id).await?;
  Ok(Json(variant))
}

pub async fn record_reward(
  State(state): State<Arc<AppState>>,
  Path(experiment_id): Path<uuid::Uuid>,
  Json(payload): Json<RewardPayload>,
) -> Result<StatusCode, Error> {
  let bandit = BanditService::new(state.redis.clone(), state.db.clone());
  bandit.record_reward(experiment_id, payload.variant_id, payload.reward).await?;
  Ok(StatusCode::OK)
}