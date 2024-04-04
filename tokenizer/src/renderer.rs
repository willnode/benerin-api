

use crate::{types::Lexicon, Tokenizer};

impl Tokenizer {
    pub fn render(&self, graph: &Vec<Lexicon>) -> String {
        let mut text = String::new();
        for lexicon in graph {
            text.push_str(&lexicon.prefix);
            for lexeme in lexicon.lexemes.iter() {
                text.push_str(&lexeme.word);
                text.push_str(&lexeme.suffix);
            }
            text.push_str(&lexicon.suffix);
        }
        text
    }
}
