use std::borrow::{Borrow, Cow};

use serde::Deserialize;

use crate::models::{
    api_error::Error,
    base::{Endpoint, RequestContext},
};

pub struct Client {
    client: reqwest::Client,
    token: Cow<'static, str>,
}

impl Client {
    pub fn new(token: impl Into<Cow<'static, str>>) -> Self {
        Self {
            client: reqwest::Client::default(),
            token: token.into(),
        }
    }

    pub fn new_with_client(token: impl Into<Cow<'static, str>>, client: reqwest::Client) -> Self {
        Self {
            client,
            token: token.into(),
        }
    }

    pub async fn query<T>(&self, params: impl Into<RequestContext<T>>) -> Result<T, Error>
    where
        T: Endpoint + for<'de> Deserialize<'de>,
    {
        let request_context: RequestContext<T> = params.into();

        let response = self
            .client
            .get(T::endpoint())
            .header::<&'static str, &str>("Authorization", self.token.borrow())
            .headers(request_context.headers)
            .body(request_context.body)
            .send()
            .await
            .map_err(Error::from)?;

        response.json::<T>().await.map_err(Error::from)
    }
}
