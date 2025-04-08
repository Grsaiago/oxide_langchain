use async_trait::async_trait;
use serde_json::Value;
use std::error::Error;

#[async_trait]
pub trait ARunnable {
    async fn ainvoke(&self, input: Value) -> Result<Value, Box<dyn Error>>;
}

#[async_trait]
impl<T> ARunnable for Vec<T>
where
    T: ARunnable + Sync + Send,
{
    async fn ainvoke(&self, input: Value) -> Result<Value, Box<dyn Error>> {
        let mut acc = input;
        for item in self.iter() {
            acc = item.ainvoke(acc).await?;
        }
        Ok(acc)
    }
}
