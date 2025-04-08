use oxide_langchain_core::Runnable;
use serde_json::Value;
use std::error::Error;

pub fn runnable_from_fn<T>(function: T) -> Box<dyn Runnable>
where
    T: FnMut(Value) -> Result<Value, Box<dyn Error>> + 'static,
{
    Box::new(function) as Box<dyn Runnable>
}
