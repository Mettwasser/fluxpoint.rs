use super::macros::{impl_noargs, model};

model! {
    :"A random dad joke"
    DadJoke: "/dadjoke";
    joke: String,
}

impl_noargs!(DadJoke);

#[cfg(test)]
mod test {
    use super::DadJoke;
    use crate::{client::Client, error::Error, models::base::NoArgs};

    #[tokio::test]
    async fn test_dadjoke() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client.fetch::<DadJoke>(NoArgs).await {
            Ok(_dadjoke) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
