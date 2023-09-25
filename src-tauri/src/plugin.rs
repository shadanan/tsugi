use crate::error::TsugiError;
use crate::task::PluginTask;
use async_trait::async_trait;

#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    fn name(&self) -> String;
    async fn tasks(&self) -> Result<Vec<PluginTask>, TsugiError>;
}
