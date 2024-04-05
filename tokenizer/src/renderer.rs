use graph::Graph;

use crate::Tokenizer;

impl Tokenizer {
    pub fn render(&self, graph: &Graph) -> String {
        let mut text = String::new();
        for lexicon in &graph.lexicons {
            text.push_str(graph.get_lexicon_prefix(lexicon));
            for lexeme in lexicon.lexemes.iter() {
                text.push_str(graph.get_word(lexeme));
                text.push_str(graph.get_lexeme_suffix(lexeme));
            }
            text.push_str(graph.get_lexicon_suffix(lexicon));
        }
        text
    }

    pub fn render_flat(&self, graph: &Graph) -> String {
        let mut text = String::new();
        for lexicon in &graph.lexicons {
            for lexeme in lexicon.lexemes.iter() {
                text.push_str(graph.get_word(lexeme));
                text.push_str(" ");
            }
        }
        if text.len() > 0 {
            text.pop();
        }
        text
    }
}
