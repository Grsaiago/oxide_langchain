use serde_json::{Map, Number, Value};
use std::error::Error;

pub trait Runnable {
    fn invoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        match input {
            Value::Null => self.handle_null_input(),
            Value::Bool(input) => self.handle_bool_input(input),
            Value::Number(input) => self.handle_number_input(input),
            Value::String(input) => self.handle_string_input(input),
            Value::Array(input) => self.handle_array_input(input),
            Value::Object(input) => self.handle_object_input(input),
        }
    }

    fn handle_null_input(&mut self) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    fn handle_bool_input(&mut self, input: bool) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    fn handle_number_input(&mut self, input: Number) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    fn handle_string_input(&mut self, input: String) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    fn handle_array_input(&mut self, input: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }

    fn handle_object_input(&mut self, input: Map<String, Value>) -> Result<Value, Box<dyn Error>> {
        unimplemented!()
    }
}

impl<R> Runnable for Box<R>
where
    R: Runnable + ?Sized,
{
    fn invoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        self.invoke(input)
    }
}

impl<T> Runnable for Vec<T>
where
    T: Runnable,
{
    fn invoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        self.iter_mut()
            .try_fold(input, |input, it| -> Result<Value, Box<dyn Error>> {
                it.invoke(input)
            })
    }
}

impl<F> Runnable for F
where
    F: FnMut(Value) -> Result<Value, Box<dyn Error>>,
{
    fn invoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        self(input)
    }
}
