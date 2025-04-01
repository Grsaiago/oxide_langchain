use serde::{Deserialize, Serialize};
use std::error::Error;

pub trait ARunnable {
    type Input: Serialize + for<'de> Deserialize<'de> + Send + Sync;
    type Output: Serialize + for<'de> Deserialize<'de> + Send + Sync;

    async fn ainvoke(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>>;
}

impl<R1, R2> ARunnable for (R1, R2)
where
    R1: ARunnable,
    R2: ARunnable,
{
    type Input = R1::Input;
    type Output = R2::Output;

    async fn ainvoke(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
        let (r1, r2) = self;

        let intermediate = serde_json::to_value(r1.ainvoke(input).await?)?;

        let r2_input = serde_json::from_value::<R2::Input>(intermediate)?;

        r2.ainvoke(r2_input).await
    }
}
