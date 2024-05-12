#![allow(unused_parens)]
use crate::args_model;

use super::model;

model! {
    :"Convert HTML to Markdown by passing a string"
    HtmlToMarkdown: "/convert/html-markdown";
    markdown: String,
}

args_model! {
    HtmlToMarkdownArgs: HtmlToMarkdown;
    body: String,
}

#[cfg(test)]
mod test {
    use super::HtmlToMarkdown;
    use crate::{client::Client, error::Error};

    #[tokio::test]
    async fn test_htmltomarkdown() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client
            .fetch::<HtmlToMarkdown>("<h1>Hi bro</h1>".to_string())
            .await
        {
            Ok(_htmltomarkdown) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
