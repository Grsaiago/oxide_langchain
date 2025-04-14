mod chat_ollama;
mod chat_open_ai;
use std::collections::HashMap;

use chat_ollama::ChatOllama;
use chat_open_ai::ChatOpenAI;
use oxide_langchain::core::Runnable;
use oxide_langchain::utils::runnable_from_fn;
use serde_json::{Number, Value};

fn main() {
    let openai = ChatOpenAI::default();
    let ollama = ChatOllama::default();

    let mut runnable_map: HashMap<String, Box<dyn Runnable>> = HashMap::new();

    runnable_map.insert("ollama".to_string(), Box::new(ollama) as Box<dyn Runnable>);
    runnable_map.insert("openai".to_string(), Box::new(openai) as Box<dyn Runnable>);

    let fn1 = runnable_from_fn(|_value| Ok(Value::Null));
    let fn2 = runnable_from_fn(|value: Value| {
        if let Value::Null = value {
            return Ok(Value::Null);
        } else {
            Ok(Value::Number(Number::from_f64(64.3).unwrap()))
        }
    });
    vec![fn2, fn1];

    println!("Hello, world!");
}
