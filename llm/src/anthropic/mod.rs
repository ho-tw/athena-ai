pub mod types;

use agent_core::{AgentError, Message, Result, Role};
use async_trait::async_trait;
use communication::ApiClient;
use config::LLMConfig;

use crate::LLMProvider;

pub use types::{AnthropicMessage, MessagesRequest, MessagesResponse};

/// Anthropic LLM provider implementation
pub struct AnthropicProvider {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: usize,
    client: ApiClient,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider from configuration
    ///
    /// # Arguments
    /// * `config` - LLM configuration containing API key, model, and parameters
    ///
    /// # Returns
    /// * `Result<Self>` - New provider instance or error
    pub fn new(config: &LLMConfig) -> Result<Self> {
        Ok(Self {
            api_key: config.api_key.clone(),
            model: config.model.clone(),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            client: ApiClient::new(),
        })
    }

    /// Convert framework Message to Anthropic message format
    /// 
    /// Note: System messages are handled separately and should not be
    /// included in the messages array
    fn convert_message(message: &Message) -> Option<types::AnthropicMessage> {
        match message.role {
            Role::System => None, // System messages go in separate field
            Role::User => Some(types::AnthropicMessage {
                role: "user".to_string(),
                content: message.content.clone(),
            }),
            Role::Assistant => Some(types::AnthropicMessage {
                role: "assistant".to_string(),
                content: message.content.clone(),
            }),
        }
    }

    /// Convert multiple framework messages to Anthropic format
    /// 
    /// Separates system messages from user/assistant messages.
    /// Returns (system_message, messages_array)
    fn convert_messages(messages: &[Message]) -> (Option<String>, Vec<types::AnthropicMessage>) {
        let mut system_message: Option<String> = None;
        let mut anthropic_messages = Vec::new();

        for message in messages {
            match message.role {
                Role::System => {
                    // Combine multiple system messages if present
                    if let Some(existing) = system_message.as_mut() {
                        existing.push_str("\n\n");
                        existing.push_str(&message.content);
                    } else {
                        system_message = Some(message.content.clone());
                    }
                }
                _ => {
                    if let Some(anthropic_msg) = Self::convert_message(message) {
                        anthropic_messages.push(anthropic_msg);
                    }
                }
            }
        }

        (system_message, anthropic_messages)
    }
}

#[async_trait]
impl LLMProvider for AnthropicProvider {
    async fn send_message(&self, messages: &[Message]) -> Result<String> {
        // Convert framework messages to Anthropic format, separating system messages
        let (system, anthropic_messages) = Self::convert_messages(messages);

        // Build the request
        let request = MessagesRequest {
            model: self.model.clone(),
            messages: anthropic_messages,
            system,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        // Call Anthropic API
        let url = "https://api.anthropic.com/v1/messages";
        
        // Create a custom client with required headers
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .timeout(self.client.timeout())
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    AgentError::LLMProvider(format!("Anthropic API request timeout: {}", e))
                } else if e.is_connect() {
                    AgentError::LLMProvider(format!("Anthropic API connection error: {}", e))
                } else if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) {
                    AgentError::LLMProvider("Anthropic API authentication failed: Invalid API key".to_string())
                } else if e.status() == Some(reqwest::StatusCode::TOO_MANY_REQUESTS) {
                    AgentError::LLMProvider("Anthropic API rate limit exceeded".to_string())
                } else {
                    AgentError::LLMProvider(format!("Anthropic API request failed: {}", e))
                }
            })?;

        // Check for HTTP errors
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            
            return Err(AgentError::LLMProvider(format!(
                "Anthropic API HTTP {} error: {}",
                status, error_text
            )));
        }

        // Deserialize the response
        let messages_response: MessagesResponse = response.json().await.map_err(|e| {
            AgentError::LLMProvider(format!("Failed to deserialize Anthropic response: {}", e))
        })?;

        // Extract the response text from content[0].text
        messages_response
            .content
            .first()
            .map(|content| content.text.clone())
            .ok_or_else(|| {
                AgentError::LLMProvider("Anthropic response contained no content".to_string())
            })
    }
}
