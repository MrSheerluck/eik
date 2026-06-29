use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::future::{Future, IntoFuture};
use std::pin::Pin;

pub struct Client {
    base_url: String,
    api_key: String,
    model: String,
}

pub struct CompletionOutput {
    pub text: String,
}

pub struct CompletionBuilder {
    base_url: String,
    api_key: String,
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f64,
    top_p: Option<f64>,
    stop: Option<Vec<String>>,
}

impl CompletionBuilder {
    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = value;
        self
    }

    pub fn max_tokens(mut self, value: u32) -> Self {
        self.max_tokens = value;
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn stop(mut self, sequences: Vec<String>) -> Self {
        self.stop = Some(sequences);
        self
    }
}

impl IntoFuture for CompletionBuilder {
    type Output = Result<CompletionOutput>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let request_body = ChatCompletionRequest {
                model: &self.model,
                messages: vec![Message {
                    role: "user",
                    content: &self.prompt,
                }],
                max_tokens: self.max_tokens,
                temperature: self.temperature,
                top_p: self.top_p,
                stop: self.stop.as_deref(),
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
                .and_then(|c| c.message.content)
                .unwrap_or_default();

            Ok(CompletionOutput { text })
        })
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    max_tokens: u32,
    temperature: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<&'a [String]>,
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
    content: Option<String>,
}

impl Client {
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            model: model.into(),
        }
    }

    pub fn complete(&self, prompt: &str) -> CompletionBuilder {
        CompletionBuilder {
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
            model: self.model.clone(),
            prompt: prompt.to_owned(),
            max_tokens: 256,
            temperature: 1.0,
            top_p: None,
            stop: None,
        }
    }
}
