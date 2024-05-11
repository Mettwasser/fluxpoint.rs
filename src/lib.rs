pub mod client;
pub mod models;

pub(crate) use crate::models::macros::args_model;

pub mod prelude {
    pub use crate::models::base::NoArgs;
    pub use crate::models::dad_joke::DadJoke;
}

pub use reqwest;

#[cfg(test)]
mod test {

    use crate::{
        client::Client,
        models::{
            api_error::Error,
            cs_datetime_format::CSDateTimeFormat,
            snowflake_date::{SnowflakeDate, SnowflakeDateArgs},
        },
        prelude::{DadJoke, NoArgs},
    };

    #[tokio::test]
    async fn test_dadjoke() -> Result<(), Error> {
        let client = Client::new("TOKEN HERE");

        let dadjoke: DadJoke = client.query(NoArgs).await?;

        println!("Joke: {:?}", dadjoke);
        Ok(())
    }

    #[tokio::test]
    async fn test_snowflake_date() -> Result<(), Error> {
        let client = Client::new("TOKEN HERE");

        let args = SnowflakeDateArgs::from((350749990681051149, CSDateTimeFormat::Basic));

        let dadjoke: SnowflakeDate = client
            .query(args) // can also put (350749990681051149, CSDateTimeFormat::Basic) directly in there
            .await?;

        println!("SnowflakeDate: {:?}", dadjoke);
        Ok(())
    }
}
