use crate::{context::HttpResponse, request::HttpRequest};
use serde::Serialize;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

// HttpRequest types

#[derive(Debug, Clone, PartialEq)]
pub enum RequestBodyType {
    JSON,
    TEXT,
    FORM,
}

impl Copy for RequestBodyType {}

#[derive(Debug, Clone)]
pub enum RequestBodyContent {
    TEXT(String),
    JSON(serde_json::Value),
    FORM(String),
}

// HttpResponse types

#[derive(PartialEq, Debug, Clone)]
pub enum ResponseContentType {
    JSON,
    TEXT,
}

#[derive(Serialize, PartialEq)]
pub(crate) enum ResponseContentBody {
    JSON(serde_json::Value),
    TEXT(String),
}

impl ResponseContentBody {
    pub fn new_text<T: Into<String>>(text: T) -> Self {
        ResponseContentBody::TEXT(text.into())
    }
}

#[derive(Debug)]
pub enum HttpResponseError {
    MissingHeader(String)
}

impl std::fmt::Display for HttpResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpResponseError::MissingHeader(header) => write!(f, "Missing header: {}", header),
        }
    }
}


// App types

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum HttpMethods {
    GET,
    PUT,
    POST,
    DELETE,
    PATCH,
}

pub struct Next;

impl Next {
    pub fn new<F: Fn(HttpRequest)>(_closure: F) -> Self {
        Next {}
    }
}

pub type Fut = Pin<Box<dyn Future<Output = HttpResponse> + Send + 'static>>;
pub type Handler = Arc<dyn Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static>;
pub type Middleware = Arc<dyn Fn(HttpRequest, HttpResponse, Next) -> Fut + Send + Sync + 'static>;
pub(crate) type Routes = HashMap<&'static str, HashMap<HttpMethods, Handler>>;
