use crate::{
    context::HttpResponse,
    request::HttpRequest,
    types::{Fut, Next},
};

/// Configuration for the Logger Middleware
///
/// ## Fields
///
/// * `method` -  Wheather to log the method
/// * `path` - Whether to log the path
/// * `duration` - Whether to log the duration

#[derive(Clone)]
pub struct LoggerConfig {
    pub method: bool,
    pub path: bool,
    pub duration: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        LoggerConfig {
            duration: true,
            method: true,
            path: true,
        }
    }
}

/// Builtin Logger Middleware
///
/// ## Arguments
///
/// * `config` - Configuration for the middleware
///
/// ## Examples
///
/// ```
/// use ripress::{app::App, middlewares::logger::logger};
/// let mut app = App::new();
/// app.use_middleware("", logger(None))
///
///```
///```
/// use ripress::{app::App, middlewares::cors::{logger, LoggerConfig}};
/// let mut app = App::new();
/// app.use_middleware("", logger(LoggerConfig {
///     duration: true,
///     method: true,
///     path: true,
/// }))
///
/// ```
pub fn logger(
    config: Option<LoggerConfig>,
) -> impl Fn(HttpRequest, HttpResponse, Next) -> Fut + Send + Sync + Clone + 'static {
    move |req, res, next| {
        let config = config.clone().unwrap_or_default();

        let start_time = std::time::Instant::now();
        let path = req.get_path().to_string();

        Box::pin(async move {
            let method = req.get_method();

            let res = next.run(req.clone(), res).await;
            let duration = start_time.elapsed();

            if config.path {
                print!("path: {}, ", path);
            }

            if config.duration {
                print!("Time taken: {}ms, ", duration.as_millis());
            }

            if config.method {
                print!("method: {}", method);
            }

            println!("");

            res
        })
    }
}
