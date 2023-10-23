use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Task {
    pub id: String,
    pub kind: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub requestor: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct GetTasksResponse {
    pub tasks: Vec<Task>,
}
