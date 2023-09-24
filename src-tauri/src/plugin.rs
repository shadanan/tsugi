use crate::error::TsugiError;
use crate::task::Task;
use async_trait::async_trait;

#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    async fn get_tasks(&self) -> Result<Vec<Task>, TsugiError>;
}
