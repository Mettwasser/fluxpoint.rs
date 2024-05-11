use std::marker::PhantomData;

use reqwest::{header::HeaderMap, Body};

pub trait Endpoint {
    fn endpoint() -> &'static str;
}

pub struct NoArgs;

pub struct RequestContext<T: Endpoint> {
    pub body: Body,
    pub headers: HeaderMap,
    _t: PhantomData<T>,
}

impl<T: Endpoint> RequestContext<T> {
    pub fn new() -> Self {
        Self {
            body: Default::default(),
            headers: Default::default(),
            _t: PhantomData,
        }
    }
}

impl<T: Endpoint> From<(Body, HeaderMap)> for RequestContext<T> {
    fn from(value: (Body, HeaderMap)) -> Self {
        Self {
            body: value.0,
            headers: value.1,
            _t: PhantomData,
        }
    }
}

impl<T: Endpoint> Default for RequestContext<T> {
    fn default() -> Self {
        Self::new()
    }
}
