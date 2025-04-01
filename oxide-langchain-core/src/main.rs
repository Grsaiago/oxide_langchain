use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Debug, Display},
};

mod traits;
use traits::{ARunnable, Runnable};

#[derive(Debug)]
pub struct RunError;

impl Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error on run")
    }
}
impl Error for RunError {}

struct OpenAiChat;
impl Runnable for OpenAiChat {
    type Input = u32;
    type Output = String;

    fn invoke(&self, input: u32) -> Result<String, Box<dyn Error>> {
        let string = format!("The number is {input}");
        println!("{string}");
        Ok(string)
    }
}

struct OllamaAiChat;
impl Runnable for OllamaAiChat {
    type Input = String;
    type Output = u32;

    fn invoke(&self, input: String) -> Result<u32, Box<dyn Error>> {
        println!("The len is {}", input.len() as u32);
        Ok(input.len() as u32)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MeuTipo {
    nome: String,
}

struct InputMeuTipo;
impl Runnable for InputMeuTipo {
    type Input = MeuTipo;
    type Output = String;

    fn invoke(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
        Ok(input.nome)
    }
}

fn recebe_um_runnable<I, O>(
    input: I,
    obj: Box<dyn Runnable<Input = I, Output = O>>,
) -> Result<O, Box<dyn Error>>
where
    I: Serialize + for<'de> Deserialize<'de>,
    O: Serialize + for<'de> Deserialize<'de>,
{
    obj.invoke(input)
}

fn main() {
    let openai = OpenAiChat {};
    let ollama = OllamaAiChat {};

    let runnable_sequence = (openai, ollama);
    println!(
        "O resultado de 10 é: {}",
        runnable_sequence.invoke(10).unwrap()
    );
}
