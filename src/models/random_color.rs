use super::core::{impl_noargs, model};

model! {
    :"A Random color"
    RandomColor: "/color/random";

    :"The Hex of the color"
    hex: String,

    :"The name of the color"
    name: String,

    :"The red parts of this color"
    r: u8,

    :"The green parts of this color"
    g: u8,

    :"The blue of this color"
    b: u8,
}

impl_noargs!(RandomColor);

#[cfg(test)]
mod test {
    use super::RandomColor;
    use crate::{client::Client, error::Error, models::core::NoArgs};

    #[tokio::test]
    async fn test_randomcolor() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client.fetch::<RandomColor>(NoArgs).await {
            Ok(_randomcolor) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
