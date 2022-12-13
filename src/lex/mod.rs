extern crate rocket;
use std::collections::HashMap;

use futures::lock::Mutex;
use once_cell::sync::Lazy;
use rocket::serde::Serialize;

pub mod parser;
pub mod renderer;
pub mod validator;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Lexicon<'a> {
    pub lexemes: Vec<Lexeme<'a>>,
    pub corrections: Vec<Correction<'a>>,
    pub prefix: &'a str,
    pub suffix: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Lexeme<'a> {
    pub word: &'a str,
    pub kind: &'a str,
    pub suffix: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Correction<'a> {
    pub start_lexeme: usize,
    pub end_lexeme: usize,
    pub r#type: &'a str,
    pub suggestion: Option<String>,
}

pub static KIND_MAP: Lazy<Mutex<HashMap<&str, &str>>> = Lazy::new(|| {
    let m = HashMap::from([
        ("", ""),
        ("a", "a"),
        ("adv", "adv"),
        ("n", "n"),
        ("p", "p"),
        ("pron", "pron"),
        ("v", "v"),
        ("infiks", "infiks"),
        ("prefiks", "prefiks"),
        ("sufiks", "sufiks"),
    ]);
    Mutex::new(m)
});

// Correction kind:
// 1: Unknown (unimplemented)
// 2: Wrong spelling
// 3: Unknown word
