use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct Client {
    base_url: String,
    api_key: String,
}

pub struct CompletionOutput {
    pub text: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    max_tokens: u32,
    temperature: f64,
}

#[derive(Debug, Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
struct AssistantMessage {
    content: String,
}

impl Client {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
        }
    }

    pub async fn complete(&self, prompt: &str) -> Result<CompletionOutput> {
        let request_body = ChatCompletionRequest {
            model: "gpt-3.5-turbo",
            messages: vec![Message {
                role: "user",
                content: prompt,
            }],
            max_tokens: 256,
            temperature: 1.0,
        };

        let url = format!("{}/chat/completions", self.base_url);

        let http_client = reqwest::Client::new();

        let response: ChatCompletionResponse = http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        let text = response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .unwrap_or_default();

        Ok(CompletionOutput { text })
    }
}
