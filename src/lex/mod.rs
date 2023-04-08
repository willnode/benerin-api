extern crate rocket;

use rocket::serde::Serialize;

pub mod parser;
pub mod renderer;
pub mod validator;
pub mod statics;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Lexicon<'a> {
    pub lexemes: Vec<Lexeme<'a>>,
    pub corrections: Vec<Correction<'a>>,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub prefix: &'a str,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub suffix: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Lexeme<'a> {
    pub word: &'a str,
    pub kind: &'a str,
    #[serde(skip_serializing_if = "str_is_empty")]
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

fn str_is_empty<'a>(metadata: &'a str) -> bool {
    match metadata {
      "" => true,
      _ => false
    }
  }

// Correction kind:
// 1: Unknown (unimplemented)
// 2: Wrong spelling
// 3: Unknown word
