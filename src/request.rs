use crate::types::{RequestBodyContent, RequestBodyType};
use futures_util::stream::StreamExt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct RequestBody {
    content: RequestBodyContent,
    content_type: RequestBodyType,
}

/// Represents an incoming HTTP request.
///
/// This struct holds various properties of an HTTP request, such as
/// query parameters, request body, HTTP method, and client IP address.
///
/// # Example
/// ```
/// use ripress::context::HttpRequest;
///
/// let req = HttpRequest::new();
/// println!("Request method: {}", req.get_method());
/// println!("Client IP: {}", req.ip().unwrap());
/// ```
///
/// # Fields
/// - `params`: Stores dynamic route parameters extracted from the URL.
/// - `queries`: Stores query parameters from the request URL.
/// - `body`: Contains the request body, which may be JSON, text, or form data.
/// - `ip`: The client's IP address.
/// - `method`: The HTTP method used (e.g., GET, POST, PUT, DELETE).
/// - `origin_url`: The full URL of the incoming request.
/// - `path`: The requested endpoint path.

#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// Dynamic route parameters extracted from the URL.
    params: HashMap<String, String>,

    /// Query parameters from the request URL.
    queries: HashMap<String, String>,

    /// The request body, which may contain JSON, text, or form data.
    body: RequestBody,

    /// The IP address of the client making the request.
    ip: String,

    /// The HTTP method used for the request (e.g., GET, POST, PUT, DELETE).
    method: String,

    /// The full URL of the incoming request.
    origin_url: String,

    /// The requested endpoint path.
    path: String,

    /// The request's headers
    headers: HashMap<String, String>,

    /// The request's cookies
    cookies: HashMap<String, String>,
}

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            queries: HashMap::new(),
            body: RequestBody {
                content_type: RequestBodyType::TEXT,
                content: RequestBodyContent::TEXT(String::new()),
            },
            ip: String::new(),
            method: String::new(),
            origin_url: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
        }
    }

    /// Checks if the `Content-Type` of the request matches the specified type.
    /// # Example
    /// ```
    /// use ripress::types::RequestBodyType;
    /// let req = ripress::context::HttpRequest::new();
    ///
    /// if req.is(RequestBodyType::JSON) {
    ///     println!("Request is JSON");
    /// }
    /// ```
    ///
    /// Returns `true` if the `Content-Type` matches, otherwise `false`.

    pub fn is(&self, content_type: RequestBodyType) -> bool {
        self.body.content_type == content_type
    }

    /// Returns the request's method (GET, POST, etc.)
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// req.get_method(); // returns (GET, POST, etc.)
    /// ```

    pub fn get_method(&self) -> String {
        self.method.to_string()
    }

    /// Returns the request's origin URL.
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// req.get_origin_url();
    /// ```
    /// For example the request is made to /user/{id} and the id is 123, the origin URL will be /user/123
    /// If the request is made to /user/123 with query params ?q=hello, the origin url will be /user/123?q=hello
    /// Returns an `Option<String>`, where `Some(url)` contains the origin_url is available, or `None` if it cannot be determined.

    pub fn get_origin_url(&self) -> Option<String> {
        Some(self.origin_url.to_string())
    }

    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// let cookie = req.get_cookie("value").unwrap();
    /// println!("cookie: {}", cookie);
    /// ```
    /// This function returns the value of the specified cookie.

    pub fn get_cookie(&self, name: &str) -> Option<String> {
        self.cookies.get(name).map(|c| c.to_string())
    }

    /// Returns the request's path.
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// req.get_path();
    /// ```
    /// For example the request is made to /user/{id} and the id is 123, the origin URL will be /user/123
    /// If the request is made to /user/123 with query params ?q=hello, the origin url will be /user/123
    /// Returns an `Option<String>`, where `Some(path)` contains the path is available, or `None` if it cannot be determined.

    pub fn get_path(&self) -> Option<String> {
        Some(self.path.to_string())
    }

    /// Returns the client's IP address.
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// let ip = req.ip();
    /// println!("Client IP: {:?}", ip);
    /// ```
    ///
    /// This function retrieves the IP address of the client making the request.
    /// Returns an `Option<String>`, where `Some(ip)` contains the IP if available, or `None` if it cannot be determined.

    pub fn ip(&self) -> Option<String> {
        Some(self.ip.to_string())
    }

    /// Returns url parameters.
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// let id = req.get_params("id");
    /// println!("Id: {:?}", id);
    /// ```
    ///
    /// This function returns the value of the specified parameter from the URL.
    /// Returns an `Option<String>`, where `Some(id)` contains the id if available, or `None` if it cannot be determined.

    pub fn get_params(&self, param_name: &str) -> Option<String> {
        self.params.get(param_name).map(|v| v.to_string())
    }

    /// Returns header based on the key.
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// let header = req.get_header("id");
    /// println!("header: {:?}", header.unwrap());
    /// ```
    ///
    /// This function returns the value of the specified header.
    pub fn get_header(&self, header_name: &str) -> Option<&String> {
        self.headers.get(&header_name.to_string())
    }
    /// Returns query parameters.
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// let id = req.get_query("id");
    /// println!("Id: {:?}", id);
    /// ```
    ///
    /// This function returns the value of the specified parameter from the URL.
    /// Returns an `Option<String>`, where `Some(id)` contains the id if available, or `None` if it cannot be determined.

    pub fn get_query(&self, query_name: &str) -> Option<String> {
        self.queries.get(query_name).map(|v| v.to_string())
    }

    /// Returns request's json body.
    ///
    /// # Example
    /// ```no_run
    /// #[derive(serde::Deserialize, serde::Serialize)]
    /// struct MyStruct {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// let req = ripress::context::HttpRequest::new();
    /// let body = req.json::<MyStruct>().unwrap();
    /// println!("name: {:?}", body.name);
    /// println!("age : {:?}", body.age);
    /// ```
    ///
    /// This function returns the json body of the request.
    /// Returns an `Result<J>`, where `Ok(J)` contains the body if it is valid json, or `Err(error)` if it is not.

    pub fn json<J>(&self) -> Result<J, String>
    where
        J: serde::de::DeserializeOwned + serde::Serialize,
    {
        let body = &self.body;

        if body.content_type == RequestBodyType::JSON {
            if let RequestBodyContent::JSON(ref json_value) = body.content {
                match serde_json::from_value::<J>(json_value.clone()) {
                    Ok(serialized) => Ok(serialized),
                    Err(e) => Err(format!("Failed to deserialize JSON: {}", e)),
                }
            } else {
                Err(String::from("Invalid JSON content"))
            }
        } else {
            Err(String::from("Wrong body type"))
        }
    }

    /// Returns request's text body.
    ///
    /// # Example
    /// ```
    /// let req = ripress::context::HttpRequest::new();
    /// let text = req.text().unwrap();
    /// println!("text : {:?}", text);
    /// ```
    ///
    /// This function returns the text body of the request.
    /// Returns an `Result<String>`, where `Ok(String)` contains the body if it is valid text, or `Err(error)` if it is not.

    pub fn text(&self) -> Result<String, String> {
        let body = &self.body;

        if body.content_type == RequestBodyType::TEXT {
            if let RequestBodyContent::TEXT(ref text_value) = body.content {
                Ok(text_value.clone())
            } else {
                Err(String::from("Invalid text content"))
            }
        } else {
            Err(String::from("Wrong body type"))
        }
    }

    /// Returns request's form_data body.
    ///
    /// # Example
    /// ```no_run
    /// let req = ripress::context::HttpRequest::new();
    /// // Let' say form data was sent as key=value and key2=value2
    /// let form_data = req.form_data().unwrap();
    /// println!("key = : {:?}", form_data.get("key"));
    /// println!("key2 = : {:?}", form_data.get("key2"));
    /// ```
    ///
    /// This function returns a HashMap of the form data.
    /// Returns an `Result<HashMap<String, String>>`, where `Ok(HashMap<String, String>)` contains the form_data if it is valid form data, or `Err(error)` if it is not.

    pub fn form_data(&self) -> Result<HashMap<String, String>, String> {
        let mut form_data: HashMap<String, String> = HashMap::new();
        let body = &self.body;

        if body.content_type == RequestBodyType::FORM {
            if let RequestBodyContent::FORM(ref text_value) = body.content {
                text_value.split("&").for_each(|pair| {
                    if let Some((key, value)) = pair.split_once("=") {
                        form_data.insert(key.to_string(), value.to_string());
                    }
                });
                Ok(form_data)
            } else {
                Err(String::from("Invalid form content"))
            }
        } else {
            Err(String::from("Wrong body type"))
        }
    }

    pub async fn from_actix_request(
        req: actix_web::HttpRequest,
        mut payload: actix_web::web::Payload,
    ) -> Result<Self, actix_web::Error> {
        // Extract all necessary data from the request early
        let mut queries = HashMap::new();
        let query_string = req.query_string();
        if !query_string.is_empty() {
            query_string.split("&").for_each(|pair| {
                if let Some((key, value)) = pair.split_once("=") {
                    queries.insert(key.to_string(), value.to_string());
                }
            });
        }

        let ip = get_real_ip(&req);
        let method = req.method().to_string();
        let origin_url = req.uri().to_string();
        let path = req.path().to_string();

        let mut cookies: HashMap<String, String> = HashMap::new();

        req.cookies().iter().for_each(|cookie| {
            cookies.insert(cookie[0].name().to_string(), cookie[0].value().to_string());
        });

        let mut headers: HashMap<String, String> = HashMap::new();

        req.headers().iter().for_each(|(key, value)| {
            headers.insert(key.to_string(), value.to_str().unwrap().to_string());
        });

        let params: HashMap<String, String> = req
            .match_info()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        let content_type = determine_content_type(&req);

        // Read the body
        let mut body = actix_web::web::BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk?;
            if (body.len() + chunk.len()) > 262_144 {
                return Err(actix_web::error::ErrorBadRequest("Body too large"));
            }
            body.extend_from_slice(&chunk);
        }

        let request_body = match content_type {
            RequestBodyType::FORM => {
                let body_string = match std::str::from_utf8(&body) {
                    Ok(s) => s.to_string(),
                    Err(_) => {
                        return Err(actix_web::error::ErrorBadRequest("Invalid UTF-8 sequence"));
                    }
                };

                RequestBody {
                    content: RequestBodyContent::FORM(body_string),
                    content_type: RequestBodyType::FORM,
                }
            }
            RequestBodyType::JSON => {
                let body_json = match std::str::from_utf8(&body) {
                    Ok(s) => match serde_json::from_str(s) {
                        Ok(json) => json,
                        Err(e) => {
                            return Err(actix_web::error::ErrorBadRequest(format!(
                                "Invalid JSON: {}",
                                e
                            )));
                        }
                    },
                    Err(_) => {
                        return Err(actix_web::error::ErrorBadRequest("Invalid UTF-8 sequence"));
                    }
                };

                RequestBody {
                    content: RequestBodyContent::JSON(body_json),
                    content_type: RequestBodyType::JSON,
                }
            }
            RequestBodyType::TEXT => {
                let body_string = match std::str::from_utf8(&body) {
                    Ok(s) => s.to_string(),
                    Err(_) => {
                        return Err(actix_web::error::ErrorBadRequest("Invalid UTF-8 sequence"));
                    }
                };

                RequestBody {
                    content: RequestBodyContent::TEXT(body_string),
                    content_type: RequestBodyType::TEXT,
                }
            }
        };

        Ok(HttpRequest {
            params,
            queries,
            body: request_body,
            ip,
            method,
            origin_url,
            path,
            headers,
            cookies,
        })
    }
}

fn determine_content_type(req: &actix_web::HttpRequest) -> RequestBodyType {
    if let Some(content_type) = req.headers().get("content-type") {
        if let Ok(content_type_str) = content_type.to_str() {
            if content_type_str.contains("application/json") {
                return RequestBodyType::JSON;
            } else if content_type_str.contains("application/x-www-form-urlencoded") {
                return RequestBodyType::FORM;
            }
        }
    }
    RequestBodyType::TEXT
}

fn get_real_ip(req: &actix_web::HttpRequest) -> String {
    req.headers()
        .get("X-Forwarded-For")
        .and_then(|val| val.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .unwrap_or_else(|| {
            req.peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or("unknown".to_string())
        })
}

#[cfg(test)]
impl HttpRequest {
    pub fn set_query(&mut self, key: &str, value: &str) {
        self.queries.insert(key.to_string(), value.to_string());
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_cookie(&mut self, key: &str, value: &str) {
        self.cookies.insert(key.to_string(), value.to_string());
    }

    pub fn set_param(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    pub fn set_json<J>(&mut self, json: J)
    where
        J: serde::de::DeserializeOwned + serde::Serialize,
    {
        self.body.content_type = RequestBodyType::JSON;
        self.body.content = RequestBodyContent::JSON(serde_json::to_value(json).unwrap());
    }

    pub fn set_text(&mut self, text: &str) {
        self.body.content_type = RequestBodyType::TEXT;
        self.body.content = RequestBodyContent::TEXT(text.to_string());
    }

    pub fn set_form(&mut self, key: &str, value: &str) {
        self.body.content_type = RequestBodyType::FORM;

        match &mut self.body.content {
            RequestBodyContent::FORM(existing) => {
                existing.push('&');
                existing.push_str(&format!("{key}={value}"));
            }
            _ => {
                self.body.content = RequestBodyContent::FORM(format!("{key}={value}"));
            }
        }
    }
}
