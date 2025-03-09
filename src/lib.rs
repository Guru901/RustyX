use actix_web::Responder;
#[allow(unused)]
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
    routes: Vec<Route>,
}

impl App {
    pub fn new() -> App {
        return App { routes: Vec::new() };
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

    pub async fn listen(self, addr: &str) {
        println!("Server listening on {}", addr);

        let routes = self.routes;
        actix_web::HttpServer::new(move || {
            let mut app = actix_web::App::new();

            for route in &routes {
                for (method, paths) in route {
                    for (path, handler) in paths {
                        let handler_clone = handler.clone();
                        let path_clone = path.to_string();

                        let route = match method {
                            HttpMethods::GET => actix_web::web::resource(path_clone).route(
                                actix_web::web::get().to(move |req| {
                                    let handler = handler_clone.clone();
                                    async move {
                                        let req = HttpRequest {};
                                        let res = HttpResponse {
                                            status_code: 200,
                                            body: String::new(),
                                        };
                                        (handler)(req, res).await
                                    }
                                }),
                            ),
                            HttpMethods::POST => actix_web::web::resource(path_clone).route(
                                actix_web::web::post().to(move |req| {
                                    let handler = handler_clone.clone();
                                    async move {
                                        let req = HttpRequest {};
                                        let res = HttpResponse {
                                            status_code: 200,
                                            body: String::new(),
                                        };
                                        (handler)(req, res).await
                                    }
                                }),
                            ),
                            HttpMethods::PUT => actix_web::web::resource(path_clone).route(
                                actix_web::web::put().to(move |req| {
                                    let handler = handler_clone.clone();
                                    async move {
                                        let req = HttpRequest {};
                                        let res = HttpResponse {
                                            status_code: 200,
                                            body: String::new(),
                                        };
                                        (handler)(req, res).await
                                    }
                                }),
                            ),
                            HttpMethods::DELETE => actix_web::web::resource(path_clone).route(
                                actix_web::web::delete().to(move |req| {
                                    let handler = handler_clone.clone();
                                    async move {
                                        let req = HttpRequest {};
                                        let res = HttpResponse {
                                            status_code: 200,
                                            body: String::new(),
                                        };
                                        (handler)(req, res).await
                                    }
                                }),
                            ),
                        };

                        app = app.service(route);
                    }
                }
            }

            app
        })
        .bind(addr)
        .unwrap()
        .run()
        .await
        .unwrap();
    }

    fn add_route(&mut self, method: HttpMethods, path: &'static str, handler: Handler) {
        let mut route = HashMap::new();
        let mut path_handlers = HashMap::new();

        path_handlers.insert(path, handler);
        route.insert(method, path_handlers);

        self.routes.push(route);
    }
}

pub struct HttpResponse {
    pub status_code: i32,
    pub body: String,
}

impl HttpResponse {
    pub fn status(mut self, code: i32) -> Self {
        self.status_code = code;
        return self;
    }

    pub fn json(mut self, json: serde_json::Value) -> Self {
        self.body = json.to_string();
        return self;
    }

    pub fn text(mut self, text: String) -> Self {
        self.body = text;
        return self;
    }

    fn to_responder(self) -> actix_web::HttpResponse {
        let body = self.body;
        actix_web::HttpResponse::build(
            actix_web::http::StatusCode::from_u16(self.status_code as u16).unwrap(),
        )
        .body(body)
    }
}

impl Responder for HttpResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        self.to_responder()
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
