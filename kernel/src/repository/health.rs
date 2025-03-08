use async_trait::async_trait;

#[async_trait]
pub trait HealthCheckRepository: Send + Sync {
    /// データベースに接続し、接続を確立できるかどうかを確認する
    async fn check_db(&self) -> bool;
}
