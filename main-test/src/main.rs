mod chat_open_ai;
use oxide_langchain_utils::runnable_from_fn;
use serde_json::{Number, Value};

fn main() {
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
