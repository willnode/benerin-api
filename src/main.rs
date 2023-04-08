
mod api;
mod lex;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

use api::cors::CORS;
use lex::statics::KIND_MAP;

use lex::validator::run_validator;
use lex::Lexicon;
use lex::parser::run_parser;
use lex::renderer::run_renderer;
use rocket::serde::{json::Json, Serialize};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Serialize)]
#[derive(Debug)]
#[serde(crate = "rocket::serde")]
struct Task<'a> {
    status: String,
    structure: Vec<Lexicon<'a>>,
    rendered: String,
}

#[derive(Database)]
#[database("kbbi")]
struct KBBI(sqlx::MySqlPool);

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[get("/check?<text>")]
async fn kbbi(mut db: Connection<KBBI>, text: &str) -> Json<Task> {
    // turn each word into Word object
    let mut word_obj = run_parser(text);
    // get distinct word list
    let mut n_map : HashMap<&str, &str> = HashMap::new();

    for lexicon in &mut word_obj {
        for lexeme in &mut lexicon.lexemes {
           if n_map.contains_key(lexeme.word) {
                lexeme.kind = &n_map[lexeme.word][..]
           }
           let m = sqlx::query("SELECT word, kind FROM kbbi WHERE word = ?").bind(lexeme.word).fetch_one(&mut *db).await;
           if m.is_err() {
               continue;
           }
           let m = m.unwrap();
           n_map.insert(lexeme.word, KIND_MAP[m.try_get::<&str, usize>(1).unwrap()]);
           lexeme.kind = &n_map[lexeme.word][..];
        }
    }

    // join the words
    run_validator(&mut word_obj);
    let word =run_renderer(&word_obj);

    Json(Task {
        status: "success".to_string(),
        structure: word_obj,
        rendered: word,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(KBBI::init()).attach(CORS).mount("/v1", routes![kbbi, all_options])
}
