use serde_json::Value;
use std::error::Error;

pub trait HandlesBoolAsInput {
    fn handle_bool_input(&self, input: bool) -> Result<Value, Box<dyn Error>>;
}

pub trait HandlesNumberAsInput {
    fn handle_number_input(&self, input: serde_json::Number) -> Result<Value, Box<dyn Error>>;
}

pub trait HandlesStringAsInput {
    fn handle_string_input(&self, input: String) -> Result<Value, Box<dyn Error>>;
}

pub trait HandlesArrayAsInput {
    fn handle_array_input(&self, input: Vec<Value>) -> Result<Value, Box<dyn Error>>;
}

pub trait HandlesObjectAsInput {
    fn handle_object_input(
        &self,
        input: serde_json::Map<String, Value>,
    ) -> Result<Value, Box<dyn Error>>;
}
