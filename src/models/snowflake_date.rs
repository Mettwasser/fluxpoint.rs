use super::core::{args_model, model, CSDateTimeFormat};

model! {
    :"A Date(Time) string from a Discord Snowflake (ID)"
    SnowflakeDate: "/utility/snowflake-date";
    date_string: String = "content",
}

args_model! {
    SnowflakeDateArgs: SnowflakeDate;
    snowflake: u64,
    format: CSDateTimeFormat,
}

#[cfg(test)]
mod test {
    use super::SnowflakeDate;
    use crate::{client::Client, error::Error, models::core::CSDateTimeFormat};

    #[tokio::test]
    async fn test_snowflakedate() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client
            .fetch::<SnowflakeDate>((552189136099082242, CSDateTimeFormat::Basic))
            .await
        {
            Ok(_snowflakedate) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
