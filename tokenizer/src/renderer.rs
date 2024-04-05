

use graph::Graph;

use crate::Tokenizer;

impl Tokenizer {
    pub fn render(&self, graph: &Graph) -> String {
        let mut text = String::new();
        for lexicon in &graph.lexicons {
            text.push_str(&lexicon.prefix);
            for lexeme in lexicon.lexemes.iter() {
                text.push_str(&lexeme.word);
                text.push_str(&lexeme.suffix);
            }
            text.push_str(&lexicon.suffix);
        }
        text
    }

    pub fn render_flat(&self, graph: &Graph) -> String {
        let mut text = String::new();
        for lexicon in &graph.lexicons {
            for lexeme in lexicon.lexemes.iter() {
                text.push_str(&lexeme.word);
                text.push_str(" ");
            }
        }
        if text.len() > 0 {
            text.pop();
        }
        text
    }
}
