use crate::env::Env;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Postgres {
    pub pool: Arc<PgPool>,
}

impl Postgres {
    pub async fn new(env: Arc<Env>) -> Result<Self, anyhow::Error> {
        let pool = PgPool::connect(&*env.database_url).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}
