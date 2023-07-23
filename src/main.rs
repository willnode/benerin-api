use axum::{extract::Query, routing::get, Router, response::Response};
use hyper::{Method, Server, header, http::HeaderValue};
use serde::{Deserialize, Deserializer};
use std::{env, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_builder;

mod lex;
use lex::{transform, SPELLENGINE};



#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    text: String,
}

async fn kbbi(Query(params): Query<Params>) -> Response<String> {
    let body = transform(&params.text, true);
    let body_str = serde_json::to_string(&body).unwrap();
    let mut res = Response::new(body_str);
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    res
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    lazy_static::initialize(&SPELLENGINE);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/v1/check", get(kbbi).layer(cors))
        .route("/health", get(health));
    let addr_str = env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let addr: SocketAddr = addr_str.parse().unwrap();
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
