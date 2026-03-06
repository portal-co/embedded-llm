use serde::{Deserialize, Serialize};

/// Represents a tool message in JSON format.
///
/// This is used when `MessageType::Tool` is passed to `send`.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolMessage {
    pub tool_call_id: String,
    pub content: String,
}

/// Represents a tool call from the assistant.
///
/// This is serialized into the output stream when the model requests a tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}
