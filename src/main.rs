use axum::{
    response::Html,
    http::StatusCode,
    routing::get,
    Router,
    response::IntoResponse,
    extract::RawQuery,
//    response::Redirect,
};
use tower_http::services::{ServeDir};
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
//use regex::Regex;
use std::sync::Mutex;
use rand::RngExt;


/*

 */

const DELAY_MAX: f32 = 0.3;
const CACHE: &str = "templates/cache.html";
#[tokio::main]
async fn main() {
    let _ = std::fs::remove_file(CACHE);
    let static_dir = ServeDir::new("static");

    let app = Router::new()
    .route("/", get(handler))
    .route("/error", get(error_handler))
    .nest_service("/static", static_dir)
    .fallback(error_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn error_handler(RawQuery(query): RawQuery) -> impl IntoResponse {
    let html = std::fs::read_to_string("templates/404.html")
    .unwrap_or_else(|_| "<h1>Error</h1><p>{}</p>".to_string());

    let error = query
    .as_deref()
    .and_then(|q| {
        // Look for "msg=" and take everything after it until the next "&"
        q.split('&')
        .find(|part| part.starts_with("r="))
        .map(|part| &part[2..]) // Skip the "msg=" prefix
    });
    let (status, message) = match error{
        Some(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        None => (StatusCode::NOT_FOUND, "Fatal exception in http: 404"),
    };
    let html = html.replace("{}", message);
    (status, Html(html))
}



async fn handler() -> impl IntoResponse {
    // already cached -> skip
    if let Ok(cached_content) = fs::read_to_string(CACHE).await {
        return Html(cached_content).into_response();
    }
    let mut lines_block = String::from("");

    // generate the html
    let _ = jitter(-1.0); // reset jitter

    let file = File::open("data.csv").await.unwrap();

    // 2. Wrap it in an async BufReader
    let reader = BufReader::new(file);

    // 3. Use a stream-like loop to read lines
    let mut lines = reader.lines();
    let _ = lines.next_line().await; // skip first line

    while let Ok(Some(mut line)) = lines.next_line().await {
        if line.len() < 3 {
            // skip the empty line, a valid line must contain 3 colons
            continue;
        }

        let delay = popcell(&mut line).parse().unwrap_or(0.0);
        let j: f32 = jitter(delay);

        line = format!(
            r#"<div class="line" style="--delay:{}s;"><span class="timestamp">[{:>12.6}] </span><span class="subsystem">{}</span><span class="text">{}</span><a href={}>{}</a></div>"#,
            j,
            if delay == 0.0 { delay } else { j },
                popcell(&mut line),
                popcell(&mut line),
                popcell(&mut line),
                popcell(&mut line)

        );
        lines_block.push_str(&line);
    }

    let mut html = fs::read_to_string("templates/index.html").await.unwrap();
    html = html.replace("{}", &lines_block);

    let _ = fs::write(CACHE, &(html)).await;
    return Html(html).into_response();
}

fn jitter(delay: f32) -> f32 {
    static LAST: Mutex<f32> = Mutex::new(0.0); // wont run again hopefully
    let mut last = LAST.lock().unwrap();
    // reset the jitter
    if delay == -1.0 {
        *last = 0.0;
        return -1.0;
    }
    let mut rng = rand::rng();
    if *last > delay {
        *last += rng.random_range(0.1..DELAY_MAX);
    } else {
        *last = delay + rng.random_range(0.1..DELAY_MAX);
    }
    return *last;
}

fn popcell(cell: &mut String) -> String {
    println!("{}",cell);
    if cell.chars().next() == Some('"') {
        cell.remove(0); // remove quot
        match cell.find("\",") {
            Some(index) => {
                // 2. Drain from 0 to index (this removes it from 'cell')
                let popped = cell.drain(..index).collect::<String>();
                let _ = cell.drain(..2).collect::<String>(); // drain ",
                return popped;
            }
            None => {
                match cell.find("\"") {
                    Some(index) => {
                        // " str " is at end of line
                        let popped = cell.drain(..index).collect::<String>();
                        let _ = cell.drain(..1).collect::<String>(); // drain ",
                        return popped.trim_end().to_string();
                    }
                    None => {
                        // cell starts with an " literal -> readd
                        return format!(",{}",
                                       std::mem::take(cell).trim_end().to_string());
                    }
                }
            }
        }
    }
    else {
        match cell.find(",") {
            Some(index) => {
                let popped = cell.drain(..index).collect::<String>();
                let _ = cell.drain(..1).collect::<String>(); // ,
                return popped;
            }
            None => {
                // line ends here
                return std::mem::take(cell).trim_end().to_string();
            }
        }
    }
//    unreachable!("sthutup");
}
