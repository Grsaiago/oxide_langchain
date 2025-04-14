use std::error::Error;

use oxide_langchain_core::Runnable;
use reqwest::{Client, ClientBuilder};
use serde_json::{Map, Number, Value};

#[derive(Debug)]
pub struct ChatOllama {
    client: Client,
}

impl Default for ChatOllama {
    fn default() -> Self {
        let client = ClientBuilder::new()
            .user_agent("chat-gpt-oxide-rust")
            .build()
            .expect("Error building client");

        ChatOllama { client }
    }
}

impl ChatOllama {
    fn new() -> ChatOllama {
        ChatOllama::default()
    }

    fn handle_bool_input(&self, input: bool) {
        todo!()
    }

    fn handle_number_input(&self, input: Number) -> Result<Value, Box<dyn Error>> {
        todo!()
    }

    fn handle_string_input(&self, input: String) -> Result<Value, Box<dyn Error>> {
        todo!()
    }

    fn handle_array_input(&self, input: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        todo!()
    }

    fn handle_object_input(&self, input: Map<String, Value>) -> Result<Value, Box<dyn Error>> {
        todo!()
    }
}

impl Runnable for ChatOllama {
    fn invoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        match input {
            Value::Object(input) => self.handle_object_input(input),
            Value::String(input) => self.handle_string_input(input),
            Value::Number(input) => self.handle_number_input(input),
            Value::Array(input) => self.handle_array_input(input),
            _ => unimplemented!(),
        }
    }
}
