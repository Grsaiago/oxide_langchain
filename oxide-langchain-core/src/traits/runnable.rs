use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

pub trait Runnable {
    fn invoke(&self, input: Value) -> Result<Value, Box<dyn Error>>;
}

// Extension traits for type-safety (these don't break object safety of the core trait)
pub trait RunnableWithTypes<Input, Output>: Runnable
where
    Input: serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
    Output: serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
{
    fn invoke_typed(&self, input: Input) -> Result<Output, Box<dyn Error>> {
        // Convert from concrete type to Value
        let value_input = serde_json::to_value(input).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // Use the object-safe method
        let value_output = self.invoke(value_input)?;

        // Convert from Value back to concrete type
        let typed_output =
            serde_json::from_value(value_output).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(typed_output)
    }
}

// Blanket implementation for any type implementing Runnable
impl<T, Input, Output> RunnableWithTypes<Input, Output> for T
where
    T: Runnable,
    Input: serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
    Output: serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + 'static,
{
}

#[derive(Serialize, Deserialize)]
struct ChatOpenAI;

impl Runnable for ChatOpenAI {
    fn invoke(&self, input: Value) -> Result<Value, Box<dyn Error>> {
        let string = serde_json::from_value::<String>(input);
        Ok(serde_json::json!({
            "nome": "daniel vargas".to_string(),
        }))
    }
}

//impl<R1, R2> Runnable for (R1, R2)
//where
//    R1: Runnable,
//    R2: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R2 as Runnable>::Output;
//}
//
//impl<R1, R2, R3> Runnable for (R1, R2, R3)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R3 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4> Runnable for (R1, R2, R3, R4)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R4 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5> Runnable for (R1, R2, R3, R4, R5)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R5: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R5 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6> Runnable for (R1, R2, R3, R4, R5, R6)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R6 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7> Runnable for (R1, R2, R3, R4, R5, R6, R7)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R7 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8> Runnable for (R1, R2, R3, R4, R5, R6, R7, R8)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R8 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9> Runnable for (R1, R2, R3, R4, R5, R6, R7, R8, R9)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R9 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> Runnable for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//    R10: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R10 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> Runnable
//    for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//    R10: Runnable,
//    R11: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R11 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12> Runnable
//    for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//    R10: Runnable,
//    R11: Runnable,
//    R12: Runnable,
//    R12: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R12 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13> Runnable
//    for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//    R10: Runnable,
//    R11: Runnable,
//    R12: Runnable,
//    R12: Runnable,
//    R13: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R13 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14> Runnable
//    for (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//    R10: Runnable,
//    R11: Runnable,
//    R12: Runnable,
//    R12: Runnable,
//    R13: Runnable,
//    R14: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R14 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15> Runnable
//    for (
//        R1,
//        R2,
//        R3,
//        R4,
//        R5,
//        R6,
//        R7,
//        R8,
//        R9,
//        R10,
//        R11,
//        R12,
//        R13,
//        R14,
//        R15,
//    )
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//    R10: Runnable,
//    R11: Runnable,
//    R12: Runnable,
//    R12: Runnable,
//    R13: Runnable,
//    R14: Runnable,
//    R15: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R15 as Runnable>::Output;
//}
//
//impl<R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15, R16> Runnable
//    for (
//        R1,
//        R2,
//        R3,
//        R4,
//        R5,
//        R6,
//        R7,
//        R8,
//        R9,
//        R10,
//        R11,
//        R12,
//        R13,
//        R14,
//        R15,
//        R16,
//    )
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//    R4: Runnable,
//    R6: Runnable,
//    R7: Runnable,
//    R8: Runnable,
//    R9: Runnable,
//    R10: Runnable,
//    R11: Runnable,
//    R12: Runnable,
//    R12: Runnable,
//    R13: Runnable,
//    R14: Runnable,
//    R15: Runnable,
//    R16: Runnable,
//{
//    type Input = <R1 as Runnable>::Input;
//    type Output = <R16 as Runnable>::Output;
//}

//impl<R1, R2, R3> Runnable for (R1, R2, R3)
//where
//    R1: Runnable,
//    R2: Runnable,
//    R3: Runnable,
//{
//    type Input = R1::Input;
//    type Output = R3::Output;
//
//    fn invoke(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
//        let (r1, r2, r3) = self;
//
//        let intermediate1_2 = serde_json::to_value(r1.invoke(input)?)?;
//
//        let r2_input = serde_json::from_value::<R2::Input>(intermediate1_2)?;
//
//        let intermediate2_3 = serde_json::to_value(r2.invoke(r2_input)?)?;
//
//        let r3_input = serde_json::from_value::<R3::Input>(intermediate2_3)?;
//
//        r3.invoke(r3_input)
//    }
//}
