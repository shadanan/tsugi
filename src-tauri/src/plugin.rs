use crate::error::TsugiError;
use crate::task;
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Task {
    pub id: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub requestor: String,
}

impl Task {
    pub fn to(self, kind: &str) -> task::Task {
        task::Task {
            id: self.id,
            kind: kind.to_string(),
            url: self.url,
            title: self.title,
            description: self.description,
            state: self.state,
            created_at: self.created_at,
            updated_at: self.updated_at,
            closed_at: self.closed_at,
            requestor: self.requestor,
        }
    }
}

#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    fn name(&self) -> String;
    async fn tasks(&self) -> Result<Vec<Task>, TsugiError>;
}
