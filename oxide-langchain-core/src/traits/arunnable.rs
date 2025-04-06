use serde_json::Value;
use std::{error::Error, future::Future};

pub trait ARunnable {
    fn ainvoke(
        &self,
        input: Value,
    ) -> impl Future<Output = Result<Value, Box<dyn Error>>> + Send + Sync;
}
