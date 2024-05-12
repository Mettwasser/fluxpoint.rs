use derive_more::From;
use serde::Deserialize;
use std::borrow::{Borrow, Cow};

use crate::error::{ApiError, Error};
use crate::models::core::{Endpoint, RequestContext};

/// Wrapper for a Fluxpoint API token.
#[derive(From, PartialEq)]
pub struct FluxpointApiToken(pub Cow<'static, str>);

impl From<&'static str> for FluxpointApiToken {
    /// Creates a new `FluxpointApiToken` from a string slice.
    fn from(value: &'static str) -> Self {
        FluxpointApiToken(value.into())
    }
}

impl From<String> for FluxpointApiToken {
    /// Creates a new `FluxpointApiToken` from a String.
    fn from(value: String) -> Self {
        FluxpointApiToken(value.into())
    }
}

/// Client for making requests to the Fluxpoint API.
pub struct Client {
    client: reqwest::Client,
    token: FluxpointApiToken,
}

impl Client {
    /// Creates a new `Client` with the provided API token.
    ///
    /// # Arguments
    ///
    /// * `token` - The Fluxpoint API token.
    ///
    /// # Example
    ///
    /// ```
    /// use fluxpoint::prelude::Client;
    ///
    /// let client = Client::new("your_api_token");
    /// ```
    pub fn new(token: impl Into<FluxpointApiToken>) -> Self {
        Self {
            client: reqwest::Client::default(),
            token: token.into(),
        }
    }

    /// Creates a new `Client` with the provided API token and a custom reqwest client.
    ///
    /// # Arguments
    ///
    /// * `token` - The Fluxpoint API token.
    /// * `client` - A custom reqwest Client.
    ///
    /// # Example
    ///
    /// ```
    /// use fluxpoint::prelude::Client;
    /// use reqwest::Client as ReqwestClient;
    ///
    /// let reqwest_client = ReqwestClient::builder().build().unwrap();
    /// let client = Client::new_with_client("your_api_token", reqwest_client);
    /// ```
    pub fn new_with_client(token: impl Into<FluxpointApiToken>, client: reqwest::Client) -> Self {
        Self {
            client,
            token: token.into(),
        }
    }

    /// Fetches data from the Fluxpoint API.
    ///
    /// # Arguments
    ///
    /// * `params` - Request context containing endpoint details.
    ///
    /// # Returns
    ///
    /// A Result containing the deserialized response data or an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fluxpoint::prelude::{Client, DadJoke, NoArgs};
    /// use serde::Deserialize;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("your_api_token");
    ///
    ///     match client.fetch::<DadJoke>(NoArgs).await {
    ///         Ok(response) => {
    ///             println!("Response: {:?}", response);
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Error: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn fetch<T>(&self, params: impl Into<RequestContext<T>>) -> Result<T, Error>
    where
        T: Endpoint + for<'de> Deserialize<'de>,
    {
        let request_context: RequestContext<T> = params.into();

        let response = self
            .client
            .get(T::endpoint())
            .header::<&'static str, &str>("Authorization", self.token.0.borrow())
            .query(&request_context.query)
            .headers(request_context.headers)
            .body(request_context.body)
            .send()
            .await
            .map_err(Error::from)?;

        if response.status() == 200 {
            // successful response. Only thing that can fail now is the JSON deserialization.
            response.json::<T>().await.map_err(Error::from)
        } else {
            // successful response, but API failed
            let err = response.json::<ApiError>().await.map_err(Error::from)?;
            Err(Error::ApiError(err))
        }
    }
}
