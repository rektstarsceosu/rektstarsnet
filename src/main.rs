use axum::{
    response::Html,
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::services::{ServeDir};
use std::fs;
use regex::Regex;

#[tokio::main]
async fn main() {
    let static_dir = ServeDir::new("static");

    let app = Router::new()
    .route("/", get(handler))
    .nest_service("/static", static_dir)
    .fallback(handler_404);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> (StatusCode, Html<String>) {
    let html = std::fs::read_to_string("templates/404.html")
    .unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());
    return (axum::http::StatusCode::NOT_FOUND, Html(html));  // 404
}

async fn handler() -> Html<String> {
    let mut html = fs::read_to_string("templates/example.html").unwrap();
    let re = Regex::new(r#"<div class="line">\[\s*([0-9.]+)\s*\]"#).unwrap();
    html = re.replace_all(&html, // FIND ALL THE MATCH INJECT DELAY (and itself because replace_all destroys the match?)
        |caps: &regex::Captures| {
            let delay_str = &caps[1];
            let delay: f32 = delay_str.parse().unwrap_or(0.0);
            return format!(r#"<div class="line" style="--delay: {}s;"><span class="timestamp">[{:>12.6}]</span>"#,
            delay, delay);
        }
    ).to_string();

    return Html(html);
}
