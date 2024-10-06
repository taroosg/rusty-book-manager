use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::HealthCheckRepository;
use crate::database::ConnectionPool;

#[derive(new)]
pub struct HealthCheckRepositoryPool {
    db: ConnectionPool,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryPool {
    async fn check_db(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok()
    }
}
