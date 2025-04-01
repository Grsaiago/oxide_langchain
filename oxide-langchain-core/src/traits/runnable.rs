use serde::{Deserialize, Serialize};
use std::error::Error;

pub trait Runnable {
    type Input: Serialize + for<'de> Deserialize<'de>;
    type Output: Serialize + for<'de> Deserialize<'de>;

    fn invoke(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>>;
}

impl<R1, R2> Runnable for (R1, R2)
where
    R1: Runnable,
    R2: Runnable,
{
    type Input = R1::Input;
    type Output = R2::Output;

    fn invoke(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
        let (r1, r2) = self;

        let intermediate = serde_json::to_value(r1.invoke(input)?)?;

        let r2_input = serde_json::from_value::<R2::Input>(intermediate)?;

        r2.invoke(r2_input)
    }
}
