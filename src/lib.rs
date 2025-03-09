use std::{collections::HashMap, future::Future, sync::Arc};

pub type Handler = Arc<dyn Fn(HttpRequest, HttpResponse) -> Fut + Send + Sync + 'static>;
pub type Fut = Box<dyn Future<Output = HttpResponse> + Send + 'static>;

type Route = HashMap<HttpMethods, HashMap<&'static str, Handler>>;

#[derive(Eq, Hash, PartialEq)]
enum HttpMethods {
    GET,
    PUT,
    POST,
    DELETE,
}

pub struct App {
    routes: Route,
}

impl App {
    pub fn new() -> App {
        return App {
            routes: HashMap::new(),
        };
    }

    pub fn get(&mut self, path: &'static str, handler: Handler) {
        self.add_route(HttpMethods::GET, path, handler);
    }

    pub fn post(&mut self, path: &'static str, handler: Handler) {
        self.add_route(HttpMethods::POST, path, handler);
    }

    pub fn put(&mut self, path: &'static str, handler: Handler) {
        self.add_route(HttpMethods::PUT, path, handler);
    }

    pub fn delete(&mut self, path: &'static str, handler: Handler) {
        self.add_route(HttpMethods::DELETE, path, handler);
    }

    pub async fn listen(&self, addr: &str) {
        println!("Server listening on {}", addr);
        actix_web::HttpServer::new(|| actix_web::App::new())
            .bind(addr)
            .unwrap()
            .run()
            .await
            .unwrap()
    }

    fn add_route(&mut self, method: HttpMethods, path: &'static str, handler: Handler) {
        let mut hm1 = HashMap::new();

        hm1.insert(path, handler);
        self.routes.insert(method, hm1);
    }
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
