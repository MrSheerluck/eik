use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::future::{Future, IntoFuture};
use std::pin::Pin;

pub struct Client {
    http: reqwest::Client,
    base_url: String,
    auth_header: String,
    model: String,
}

pub struct CompletionOutput {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

pub struct CompletionBuilder {
    http: reqwest::Client,
    base_url: String,
    auth_header: String,
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
                messages: vec![RequestMessage {
                    role: "user",
                    content: &self.prompt,
                }],
                max_tokens: self.max_tokens,
                temperature: self.temperature,
                top_p: self.top_p,
                stop: self.stop.as_deref(),
            };

            let url = format!("{}/chat/completions", self.base_url);

            let response: ChatCompletionResponse = self
                .http
                .post(&url)
                .header("Authorization", &self.auth_header)
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
    messages: Vec<RequestMessage<'a>>,
    max_tokens: u32,
    temperature: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<&'a [String]>,
}

#[derive(Debug, Serialize)]
struct RequestMessage<'a> {
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
        let api_key = api_key.into();
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.into(),
            auth_header: format!("Bearer {api_key}"),
            model: model.into(),
        }
    }

    pub fn complete(&self, prompt: &str) -> CompletionBuilder {
        CompletionBuilder {
            http: self.http.clone(),
            base_url: self.base_url.clone(),
            auth_header: self.auth_header.clone(),
            model: self.model.clone(),
            prompt: prompt.to_owned(),
            max_tokens: 256,
            temperature: 1.0,
            top_p: None,
            stop: None,
        }
    }
}
