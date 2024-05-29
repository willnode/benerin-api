use axum::{response::{IntoResponse, Response}, routing::{get, post}, Router};
use axum::Json;
use hyper::{header::{self, ACCEPT, CONTENT_TYPE}, http::HeaderValue, Method, Server};
use serde::Deserialize;
use spellcheck::SpellCheck;
use stemmer::Stemmer;
use tokenizer::Tokenizer;
use std::{env, net::SocketAddr, sync::{Arc, Mutex}};
use tower_http::cors::{Any, CorsLayer};
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    text: String,
    #[serde(default)]
    tasks: String,
}

static FEATURES: Lazy<Arc<Mutex<(Tokenizer, Stemmer, SpellCheck)>>> = Lazy::new(|| init_features());

async fn kbbi(Json(payload): Json<Params>) -> impl IntoResponse {
    // Here you can handle the POST request, for example:
    let mutex = &*FEATURES.lock().unwrap();
    let (tokenizer, stemmer, spellchecker) = mutex;

    let mut body = tokenizer.parse(payload.text);
    let mut need_tokenized_output = false;
    for task in payload.tasks.split(",") {
        match task {
            "spellcheck" => body = spellchecker.lookup_graph(&body),
            "stemming" => body = stemmer.stem_graph(&body),
            "tokenize" => need_tokenized_output = true,
            _ => {}
        }
    }
    if need_tokenized_output {
        let body_str = serde_json::to_string(&body).unwrap();
        let mut res = Response::new(body_str);
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        res
    } else {
        let body_str = tokenizer.render(&body);
        let mut res = Response::new(body_str);
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain"),
        );
        res
    }
}

fn init_features() -> Arc<Mutex<(Tokenizer, Stemmer, SpellCheck)>> {
    let tokenizer = Tokenizer::new();
    let stemmer = Stemmer::new("");
    let spellchecker = SpellCheck::new();
    Arc::new(Mutex::new((tokenizer, stemmer, spellchecker)))
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(Any);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", post(kbbi).layer(cors))
        .route("/health", get(health));
    let addr_str = env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let addr: SocketAddr = addr_str.parse().unwrap();
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
