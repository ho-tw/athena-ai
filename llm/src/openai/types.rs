//! Type definitions for OpenAI API requests and responses.

use serde::{Deserialize, Serialize};

/// OpenAI API message format.
///
/// Represents a single message in the conversation with role and content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIMessage {
    /// The role of the message sender ("system", "user", or "assistant")
    pub role: String,
    /// The content of the message
    pub content: String,
}

/// Request structure for OpenAI Chat Completions API.
///
/// This structure is serialized to JSON and sent to the OpenAI API.
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    /// The model to use (e.g., "gpt-4", "gpt-3.5-turbo")
    pub model: String,
    /// The conversation messages
    pub messages: Vec<OpenAIMessage>,
    /// Sampling temperature (0.0 to 2.0)
    pub temperature: f32,
    /// Maximum number of tokens to generate
    pub max_tokens: usize,
}

/// Response structure from OpenAI Chat Completions API.
///
/// This structure is deserialized from the JSON response.
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    /// Unique identifier for the completion
    pub id: String,
    /// Object type (always "chat.completion")
    pub object: String,
    /// Unix timestamp of when the completion was created
    pub created: u64,
    /// The model used for the completion
    pub model: String,
    /// Array of completion choices (usually contains one element)
    pub choices: Vec<Choice>,
}

/// Individual choice in the response.
///
/// OpenAI can return multiple choices if requested, but typically returns one.
#[derive(Debug, Deserialize)]
pub struct Choice {
    /// Index of this choice in the choices array
    pub index: u32,
    /// The generated message
    pub message: OpenAIMessage,
    /// Reason why the model stopped generating (e.g., "stop", "length")
    pub finish_reason: Option<String>,
}
