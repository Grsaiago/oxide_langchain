use async_trait::async_trait;
use serde_json::{Map, Number, Value};
use std::error::Error;

#[async_trait]
pub trait ARunnable {
    async fn ainvoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        match input {
            Value::Null => self.handle_null_input().await,
            Value::Bool(input) => self.handle_bool_input(input).await,
            Value::Number(input) => self.handle_number_input(input).await,
            Value::String(input) => self.handle_string_input(input).await,
            Value::Array(input) => self.handle_array_input(input).await,
            Value::Object(input) => self.handle_object_input(input).await,
        }
    }

    async fn handle_null_input(&mut self) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    async fn handle_bool_input(&mut self, input: bool) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    async fn handle_number_input(&mut self, input: Number) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    async fn handle_string_input(&mut self, input: String) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    async fn handle_array_input(&mut self, input: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    async fn handle_object_input(
        &mut self,
        input: Map<String, Value>,
    ) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }
}

#[async_trait]
impl<T> ARunnable for Vec<T>
where
    T: ARunnable + Sync + Send,
{
    async fn ainvoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        let mut acc = input;
        for item in self.iter_mut() {
            acc = item.ainvoke(acc).await?;
        }
        Ok(acc)
    }
}
