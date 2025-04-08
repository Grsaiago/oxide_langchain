use serde_json::Value;
use std::error::Error;

pub trait Runnable {
    fn invoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>>;
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

impl<T> Runnable for T
where
    T: FnMut(Value) -> Result<Value, Box<dyn Error>>,
{
    fn invoke(&mut self, input: Value) -> Result<Value, Box<dyn Error>> {
        self(input)
    }
}
