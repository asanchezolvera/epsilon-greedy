pub struct BanditService {
  redis: redis::Client,
  db: sqlx::PgPool,
}

impl BanditService {
  pub async fn get_variant(&self, experiment_id: uuid::Uuid, session_id: &str) -> Result<Variant, Error> {
      // Check if session already has assigned variant
      let existing = self.get_session_variant(session_id, experiment_id).await?;
      if let Some(variant) = existing {
          return Ok(variant);
      }

      // Get experiment configuration
      let experiment = self.get_experiment(experiment_id).await?;
      
      // Run epsilon-greedy selection
      let variant = if rand::random::<f64>() < experiment.epsilon {
          // Exploration: random variant
          let idx = rand::random::<usize>() % experiment.variants.len();
          experiment.variants[idx].clone()
      } else {
          // Exploitation: best performing variant
          experiment.variants
              .iter()
              .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
              .unwrap()
              .clone()
      };
      
      // Store session assignment
      self.store_session_variant(session_id, experiment_id, &variant).await?;
      
      Ok(variant)
  }

  pub async fn record_reward(&self, experiment_id: uuid::Uuid, variant_id: uuid::Uuid, reward: f64) -> Result<(), Error> {
      // Update variant statistics
      let mut conn = self.db.acquire().await?;
      
      sqlx::query!(
          r#"
          UPDATE variant_stats 
          SET count = count + 1,
              value = (value * count + $1) / (count + 1)
          WHERE variant_id = $2 AND experiment_id = $3
          "#,
          reward,
          variant_id,
          experiment_id
      )
      .execute(&mut conn)
      .await?;
      
      Ok(())
  }
}