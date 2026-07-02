use crate::clients::{Client, CompletionBuilder, Message};

pub struct Agent {
    client: Client,
    system_message: Option<Message>,
}

impl Agent {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
            system_message: None,
        }
    }

    pub fn system(mut self, prompt: impl Into<String>) -> Self {
        self.system_message = Some(Message::system(prompt));
        self
    }

    pub fn prompt(&self, prompt: impl Into<String>) -> CompletionBuilder {
        let prompt: String = prompt.into();

        let mut messages = Vec::new();

        if let Some(ref system_msg) = self.system_message {
            messages.push(Message::system(system_msg.content.clone()));
        }
        messages.push(Message::user(prompt));

        self.client.chat(&messages)
    }
}
