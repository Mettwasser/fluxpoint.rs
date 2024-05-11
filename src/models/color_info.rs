use super::{base::RequestContext, macros::model};

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

pub enum ColorInfoSearchBy {
    RGB(u8, u8, u8),
    Hex(String),
    Name(String),
}

pub struct ColorInfoArgs {
    search_by: ColorInfoSearchBy,
}

impl ColorInfoArgs {
    pub fn new(search_by: ColorInfoSearchBy) -> Self {
        Self { search_by }
    }
}

impl From<ColorInfoArgs> for RequestContext<ColorInfo> {
    fn from(value: ColorInfoArgs) -> Self {
        match value.search_by {
            ColorInfoSearchBy::RGB(r, g, b) => {
                let rgb_string = format!("{},{},{}", r, g, b);
                RequestContext::from_query(vec![("rgb".into(), rgb_string.into())])
            }
            ColorInfoSearchBy::Hex(hex) => {
                RequestContext::from_query(vec![("hex".into(), hex.into())])
            }
            ColorInfoSearchBy::Name(name) => {
                RequestContext::from_query(vec![("name".into(), name.into())])
            }
        }
    }
}

impl From<ColorInfoSearchBy> for RequestContext<ColorInfo> {
    fn from(value: ColorInfoSearchBy) -> Self {
        match value {
            ColorInfoSearchBy::RGB(r, g, b) => {
                let rgb_string = format!("{},{},{}", r, g, b);
                RequestContext::from_query(vec![("rgb".into(), rgb_string.into())])
            }
            ColorInfoSearchBy::Hex(hex) => {
                RequestContext::from_query(vec![("hex".into(), hex.into())])
            }
            ColorInfoSearchBy::Name(name) => {
                RequestContext::from_query(vec![("name".into(), name.into())])
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ColorInfo, ColorInfoSearchBy};
    use crate::{client::Client, error::Error};

    #[tokio::test]
    async fn test_colorinfo() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client
            .fetch::<ColorInfo>(ColorInfoSearchBy::Hex("#00FF00".into()))
            .await
        {
            Ok(_colorinfo) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
