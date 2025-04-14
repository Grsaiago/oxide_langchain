use std::error::Error;

use oxide_langchain::core::Runnable;
use reqwest::{Client, ClientBuilder};
use serde_json::{Map, Number, Value};

#[derive(Debug)]
pub struct ChatOpenAI {
    client: Client,
}

impl Default for ChatOpenAI {
    fn default() -> Self {
        let client = ClientBuilder::new()
            .user_agent("chat-gpt-oxide-rust")
            .build()
            .expect("Error building client");

        ChatOpenAI { client }
    }
}

impl ChatOpenAI {
    fn new() -> ChatOpenAI {
        ChatOpenAI::default()
    }
}

impl Runnable for ChatOpenAI {}
