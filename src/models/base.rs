use std::{borrow::Cow, marker::PhantomData};

use reqwest::{header::HeaderMap, Body};
pub struct NoArgs;
pub type QueryLike = Vec<(Cow<'static, str>, Cow<'static, str>)>;

pub trait Endpoint {
    fn endpoint() -> &'static str;
}

pub trait JsonFromFields {
    fn get_json(&self) -> serde_json::Value;
}

pub struct RequestContext<T: Endpoint> {
    pub body: Body,
    pub headers: HeaderMap,
    pub query: QueryLike,
    _t: PhantomData<T>,
}

impl<T: Endpoint> RequestContext<T> {
    pub fn new() -> Self {
        Self {
            body: Body::default(),
            headers: HeaderMap::default(),
            query: QueryLike::default(),
            _t: PhantomData,
        }
    }

    pub fn from_query(query: QueryLike) -> Self {
        Self {
            query,
            _t: PhantomData,
            ..Default::default()
        }
    }
}

impl<T: Endpoint> From<(Body, HeaderMap, QueryLike)> for RequestContext<T> {
    fn from(value: (Body, HeaderMap, QueryLike)) -> Self {
        Self {
            body: value.0,
            headers: value.1,
            query: value.2,
            _t: PhantomData,
        }
    }
}

impl<T: Endpoint> Default for RequestContext<T> {
    fn default() -> Self {
        Self::new()
    }
}
