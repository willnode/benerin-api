#[macro_use]
extern crate rocket;
use std::collections::{HashSet};

use rocket::serde::{json::Json, Serialize};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Serialize)]
#[derive(Debug)]
#[serde(crate = "rocket::serde")]
struct Task {
    status: String,
    data: String,
}

struct Word {
    word: String,
    kind: String,
}

#[derive(Database)]
#[database("kbbi")]
struct KBBI(sqlx::MySqlPool);

#[get("/fix?<word>")]
async fn kbbi(mut db: Connection<KBBI>, word: &str) -> Json<Task> {


    // split word by spaces
    let word = word.split_whitespace().collect::<Vec<&str>>();
    // turn each word into Word object
    let mut word_obj = word.iter().map(|w| Word {
        word: w.to_string(),
        kind: "".to_string(),
    }).collect::<Vec<Word>>();
    // get distinct word list
    let m: HashSet<&str> = word.into_iter().collect();
    for name in m {
        let n = name.to_lowercase();
        let m = sqlx::query("SELECT kata, tipe FROM kbbi WHERE kata = ?").bind(n).fetch_one(&mut *db).await;
        if m.is_err() {
            continue;
        }
        let m = m.unwrap();
        let mw = Word {
            word: name.to_string(),
            kind: m.try_get(1).unwrap(),
        };
        for w in &mut word_obj {
            if w.word == mw.word {
                w.kind = mw.kind.clone();
            }
        }
    }

    // for each word, add strike tag if kind is empty
    let word = word_obj.iter().map(|w| {
        if w.kind.is_empty() {
            format!("<s>{}</s>", w.word)
        } else {
            w.word.to_string()
        }
    }).collect::<Vec<String>>();
    // join the words
    let word = word.join(" ");

    Json(Task {
        status: "success".to_string(),
        data: word,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(KBBI::init()).mount("/v1", routes![kbbi])
}
