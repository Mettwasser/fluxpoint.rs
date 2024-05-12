#![allow(unused_parens)]
use crate::{args_model, models::model};

model! {
    :"Convert Markdown to HTML by passing a string"
    MarkdownToHtml: "/convert/markdown-html";
    html: String,
}

args_model! {
    MarkdownToHtmlArgs: MarkdownToHtml;
    body: String,
}

#[cfg(test)]
mod test {
    use super::MarkdownToHtml;
    use crate::{client::Client, error::Error};

    #[tokio::test]
    async fn test_markdowntohtml() -> Result<(), Error> {
        let client = Client::new(std::env::var("FLUXPOINT_API_TOKEN").unwrap());

        match client.fetch::<MarkdownToHtml>("# Hi bro".to_string()).await {
            Ok(_markdowntohtml) => Ok(()),
            Err(why) => Err(why),
        }
    }
}
