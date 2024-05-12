use crate::models::core::macros::model;

model! {
    :"The Error Response given by the API itself."
    ApiError;

    :"Whether the response was successful or not"
    success: bool,

    :"The status code returned by this request"
    code: u16,

    :"The error message of the response. None if the returned string is empty."
    message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Happens if reqwest itself fails")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("Happens if the request goes through, but fails because of something on the API's side (e.g. Bad request)")]
    ApiError(ApiError),
}
