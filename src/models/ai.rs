use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AiMemory {
    pub player_id: uuid::Uuid,
    pub key: String,
    pub value: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
