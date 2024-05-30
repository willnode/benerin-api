use axum::Json;
use axum::{
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use axum_swagger_ui::swagger_ui;
use deepsize::DeepSizeOf;
use graph::{Graph, Lexicon};
use hyper::{
    header::{self, ACCEPT, CONTENT_TYPE},
    http::HeaderValue,
    Method, Server,
};
use once_cell::sync::Lazy;
use serde::Deserialize;
use spellcheck::SpellCheck;
use std::time::Instant;
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use stemmer::Stemmer;
use tokenizer::Tokenizer;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    text: String,
    #[serde(default)]
    tasks: Vec<String>,
    #[serde(default)]
    lexicons: Option<Vec<Lexicon>>,
}

static FEATURES: Lazy<Arc<Mutex<(Tokenizer, Stemmer, SpellCheck)>>> = Lazy::new(|| init_features());

async fn tokenizer(Json(payload): Json<Params>) -> impl IntoResponse {
    // Here you can handle the POST request, for example:
    let mutex = &*FEATURES.lock().unwrap();
    let (tokenizer, stemmer, spellchecker) = mutex;

    let mut body = match payload.lexicons {
        None => tokenizer.parse(payload.text),
        Some(t) => Graph {
            text: payload.text,
            lexicons: t,
            using_keys: true,
        },
    };
    let mut need_tokenized_output = false;
    for task in payload.tasks {
        match task.as_str() {
            "spellcheck" => body = spellchecker.lookup_graph(&body),
            "stemming" => body = stemmer.stem_graph(&body),
            "tokenize" => need_tokenized_output = true,
            "init_keys" => body.init_hash_keys(),
            _ => {}
        }
    }
    if need_tokenized_output {
        let body_str = serde_json::to_string(&body).unwrap();
        let mut res = Response::new(body_str);
        let mime = HeaderValue::from_static("application/json");
        res.headers_mut().insert(header::CONTENT_TYPE, mime);
        res
    } else {
        let body_str = tokenizer.render(&body);
        let mut res = Response::new(body_str);
        let mime = HeaderValue::from_static("text/plain");
        res.headers_mut().insert(header::CONTENT_TYPE, mime);
        res
    }
}

async fn postal(Json(payload): Json<Params>)-> impl IntoResponse {

}

fn init_features() -> Arc<Mutex<(Tokenizer, Stemmer, SpellCheck)>> {
    let start = Instant::now();
    let tokenizer = Tokenizer::new();
    let stemmer = Stemmer::new();
    let spellcheck = SpellCheck::new();
    let duration = start.elapsed();

    println!("Initialization took: {:.2?} seconds", duration);
    println!("Spellcheck heap: {:.2?} MB", spellcheck.deep_size_of() / 1024 / 1024);
    spellcheck.debug_heap();

    Arc::new(Mutex::new((tokenizer, stemmer, spellcheck)))
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    {
        let _ = &*FEATURES.lock().unwrap();
    }
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CONTENT_TYPE])
        .allow_origin(Any);

    let app = Router::new()
        .route(
            "/swagger",
            get(|| async {
                let body = swagger_ui("swagger/openapi.json");
                let mut resp = Response::new(body);
                let mime = HeaderValue::from_static("text/html");
                resp.headers_mut().insert(header::CONTENT_TYPE, mime);
                resp
            }),
        )
        .route(
            "/swagger/openapi.json",
            get(|| async {
                let body = include_str!("openapi.json").to_owned();
                let mut resp = Response::new(body);
                let mime = HeaderValue::from_static("application/json");
                resp.headers_mut().insert(header::CONTENT_TYPE, mime);
                resp
            }),
        )
        .route("/", get(Redirect::to("/swagger")))
        .route("/", post(tokenizer).layer(cors))
        .route("/health", get(health));
    let addr_str = env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let addr: SocketAddr = addr_str.parse().unwrap();
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
