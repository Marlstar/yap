use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    content: String,
    uuid: Uuid,
}
impl Message {
    pub fn new(content: String) -> Self {
        Self {
            content,
            uuid: Uuid::new_v4(),
        }
    }
}
