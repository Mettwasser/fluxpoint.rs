pub mod client;
pub mod error;
pub mod models;

pub(crate) use crate::models::macros::args_model;

pub mod prelude {
    pub use crate::{
        client::{Client, FluxpointApiToken},
        models::{base::NoArgs, dad_joke::DadJoke},
    };
}

pub use reqwest;
