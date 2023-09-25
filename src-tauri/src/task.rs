use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PluginTask {
    pub key: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub requestor: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Task {
    pub key: String,
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

impl Serialize for Task {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Task", 10)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("url", &self.url)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("updated_at", &self.updated_at)?;
        state.serialize_field("closed_at", &self.closed_at)?;
        state.serialize_field("requestor", &self.requestor)?;
        state.end()
    }
}

impl Task {
    pub fn id(&self) -> String {
        format!("{}/{}", self.kind, self.key)
    }

    pub fn from(value: PluginTask, kind: String) -> Self {
        Task {
            key: value.key,
            kind,
            url: value.url,
            title: value.title,
            description: value.description,
            state: value.state,
            created_at: value.created_at,
            updated_at: value.updated_at,
            closed_at: value.closed_at,
            requestor: value.requestor,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PluginStatus {
    pub name: String,
    pub status: String,
    pub message: String,
}

impl Serialize for PluginStatus {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("PluginStatus", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("message", &self.message)?;
        state.end()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetTasksResponse {
    pub plugins: Vec<PluginStatus>,
    pub tasks: Vec<Task>,
}

impl Serialize for GetTasksResponse {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("GetTasksResponse", 2)?;
        state.serialize_field("plugins", &self.plugins)?;
        state.serialize_field("tasks", &self.tasks)?;
        state.end()
    }
}
