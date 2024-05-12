use serde::Serialize;

use super::core::{model, RequestContext};

model! {
    :"A Random color"
    ColorInfo: "/color/info";

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

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, derive_more::Display)]
#[serde(untagged)]
pub enum Color {
    #[display(fmt = "{},{},{}", _0, _1, _2)]
    RGB(u8, u8, u8),
    #[display(fmt = "{}", _0)]
    Hex(String),
    #[display(fmt = "{}", _0)]
    Name(String),
}

pub struct ColorInfoArgs {
    search_by: Color,
}

impl ColorInfoArgs {
    pub fn new(search_by: Color) -> Self {
        Self { search_by }
    }
}

impl From<ColorInfoArgs> for RequestContext<ColorInfo> {
    fn from(value: ColorInfoArgs) -> Self {
        match value.search_by {
            Color::RGB(r, g, b) => {
                let rgb_string = format!("{},{},{}", r, g, b);
                RequestContext::from_query(vec![("rgb".into(), rgb_string.into())])
            }
            Color::Hex(hex) => RequestContext::from_query(vec![("hex".into(), hex.into())]),
            Color::Name(name) => RequestContext::from_query(vec![("name".into(), name.into())]),
        }
    }
}

impl From<Color> for RequestContext<ColorInfo> {
    fn from(value: Color) -> Self {
        match value {
            Color::RGB(r, g, b) => {
                let rgb_string = format!("{},{},{}", r, g, b);
                RequestContext::from_query(vec![("rgb".into(), rgb_string.into())])
            }
            Color::Hex(hex) => RequestContext::from_query(vec![("hex".into(), hex.into())]),
            Color::Name(name) => RequestContext::from_query(vec![("name".into(), name.into())]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Color, ColorInfo};
    use crate::{client::Client, error::Error};

    #[tokio::test]
    async fn test_colorinfo() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client
            .fetch::<ColorInfo>(Color::Hex("#00FF00".into()))
            .await
        {
            Ok(_colorinfo) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
