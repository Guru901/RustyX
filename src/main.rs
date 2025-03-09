use rustyX::{App, HttpRequest, HttpResponse};
use serde_json::json;

#[tokio::main]
async fn main() {
    let app = App::new();

    app.get("/", index);
    app.get("/user/:id", find_user);
    app.get("/search", search);

    app.listen("127.0.0.1:3000");
}

async fn index(_req: HttpRequest, res: HttpResponse) -> HttpResponse {
    return res.status(200).json(json!({
        "hehehe": "hehehe"
    }));
}

async fn find_user(req: HttpRequest, res: HttpResponse) -> HttpResponse {
    let user_id = req.get_params("id").unwrap_or(String::new());
    return res.status(200).text(format!("Hello, {user_id}"));
}

async fn search(req: HttpRequest, res: HttpResponse) -> HttpResponse {
    let q = req.get_query("q").unwrap_or(String::new());
    return res
        .status(200)
        .text(format!("Nothing found for search: {q}"));
}
