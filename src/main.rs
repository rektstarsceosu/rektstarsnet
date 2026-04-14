use axum::{
    response::Html,
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::services::{ServeDir};

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
    (axum::http::StatusCode::NOT_FOUND, Html(html))  // ← This is 404
}

async fn handler() -> Html<String> {
    let mut html = String::from(r#"<!DOCTYPE html>
    <html><head><style>
    p { opacity: 0; animation: reveal 0s ease-in forwards; }
    p:nth-child(1) { animation-delay: 0s; }
    p:nth-child(2) { animation-delay: 0.3s; }
    p:nth-child(3) { animation-delay: 0.6s; }
    @keyframes reveal { to { opacity: 1; } }
    </style></head><body>"#);

    for i in 1..=3 {
        html.push_str(&format!("<p>line {}</p>", i));
    }

    html.push_str("</body></html>");
    Html(html)
}
