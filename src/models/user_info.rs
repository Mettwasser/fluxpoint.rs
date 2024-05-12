use chrono::{DateTime, Utc};

use super::core::{impl_noargs, model};

model! {
    :"The UserInfo associated with the API Token"
    UserInfo: "/me";
    id: String,
    created: DateTime<Utc>
}

impl_noargs!(UserInfo);

#[cfg(test)]
mod test {
    use super::UserInfo;
    use crate::{client::Client, error::Error, models::core::NoArgs};

    #[tokio::test]
    async fn test_userinfo() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client.fetch::<UserInfo>(NoArgs).await {
            Ok(_userinfo) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
