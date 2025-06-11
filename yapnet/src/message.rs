use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    content: String,
    uuid: Uuid,
    sender: Uuid,
}
