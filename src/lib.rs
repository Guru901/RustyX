use std::{collections::HashMap, future::Future, sync::Arc};

pub type Handler = Arc<dyn Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static>;
pub type Fut = Box<dyn Future<Output = HttpResponse> + Send + 'static>;

type Route = HashMap<HttpMethods, HashMap<&'static str, Handler>>;

enum HttpMethods {
    GET,
    PUT,
    POST,
    DELETE,
}

pub struct App {
    routes: Vec<Route>,
}

impl App {
    pub fn new() -> App {
        return App { routes: vec![] };
    }

    pub fn get<F, Fut>(&self, path: &str, handler: F)
    where
        F: Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HttpResponse> + Send + 'static,
    {
    }

    pub fn post<F, Fut>(&self, path: &str, handler: F)
    where
        F: Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HttpResponse> + Send + 'static,
    {
    }

    pub fn put<F, Fut>(&self, path: &str, handler: F)
    where
        F: Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HttpResponse> + Send + 'static,
    {
    }

    pub fn delete<F, Fut>(&self, path: &str, handler: F)
    where
        F: Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = HttpResponse> + Send + 'static,
    {
    }

    pub fn listen(&self, addr: &str) {}
}

pub struct HttpResponse;

impl HttpResponse {
    pub fn status(self, code: i32) -> Self {
        return self;
    }

    pub fn json(self, json: serde_json::Value) -> Self {
        return self;
    }

    pub fn text(self, text: String) -> Self {
        return self;
    }
}

pub struct HttpRequest;

impl HttpRequest {
    pub fn get_params(&self, param_name: &str) -> Option<String> {
        return Some(String::new());
    }

    pub fn get_query(&self, query_name: &str) -> Option<String> {
        return Some(String::new());
    }
}
