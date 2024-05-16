pub mod client;
pub mod error;
pub mod models;

#[cfg(feature = "imagegen")]
pub mod imagegen {
    pub use crate::models::imagegen::*;
}

#[cfg(feature = "imagegen")]
pub(crate) use crate::models::imagegen::macros::state;

pub(crate) use crate::models::core::macros::args_model;

pub mod prelude {
    pub use crate::client::{Client, FluxpointApiToken};
}
