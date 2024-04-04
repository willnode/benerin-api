
use benerin_data::EMPTY_STR;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lexicon<'a> {
    pub lexemes: Vec<Lexeme<'a>>,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub prefix: &'a str,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub suffix: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lexeme<'a> {
    pub word: &'a str,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub suffix: &'a str,
}


fn str_is_empty<'a>(metadata: &'a str) -> bool {
    metadata == EMPTY_STR
}

