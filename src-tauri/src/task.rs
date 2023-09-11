use serde::ser::{Serialize, SerializeStruct, Serializer};

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

// Serialize Task as JSON
impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Task", 10)?;
        state.serialize_field("id", &self.id)?;
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
