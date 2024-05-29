use graph::{Graph, Lexeme, Lexicon};
use postemi::Postemi;
// use sastrawi::Sastrawi;
use std::collections::HashSet;
use tokenizer::Tokenizer;

mod postemi;
// mod sastrawi;

pub struct Stemmer {
    tokenizer: Tokenizer,
    stop_words: HashSet<String>,
    engine: Postemi,
    pub use_stop_words: bool,
}

impl Stemmer {
    // Initialization function
    pub fn new() -> Self {
        Stemmer {
            tokenizer: Tokenizer::new(),
            stop_words: benerin_data::get_stop_words_in_hash_set(),
            engine: Postemi::new(),
            use_stop_words: true,
        }
    }

    pub fn stem_word(&self, word: &str) -> String {
        self.engine.stem_word(word).unwrap_or(word).to_owned()
    }

    pub fn stem_word_op(&self, word: &str) -> Option<&str> {
        self.engine.stem_word(word)
    }

    pub fn stem(&self, text: &str) -> String {
        let graph = self.tokenizer.parse(text.to_owned());
        let result = self.stem_graph(&graph);
        self.tokenizer.render(&result)
    }

    pub fn stem_graph(&self, graph: &Graph) -> Graph {
        let mut g = Graph::new("".to_owned(), graph.using_keys);
        for lexicon in &graph.lexicons {
            let mut p = Lexicon::new(g.text.len());
            for lexeme in lexicon.lexemes.iter() {
                self.stem_word_lexeme(&graph, lexeme, &mut g, &mut p);
            }
            g.lexicons.push(p)
        }
        g.trim_end();
        g
    }

    fn stem_word_lexeme(&self, graph: &Graph, lexeme: &Lexeme, g: &mut Graph, p: &mut Lexicon) {
        let ow = graph.get_word(lexeme);
        match self.stem_word_op(&ow.to_ascii_lowercase()) {
            Some(s) => {
                if self.use_stop_words && self.stop_words.contains(s) {
                    return;
                }
                let mut w = g.push_word(s, graph.get_key(lexeme));
                w.set_suffix(g.push_str(" "));
                p.push_lexeme(w);
            }
            None => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let stemming = Stemmer::new();
        assert_eq!(stemming.stem("menari di sekolahan"), "tari sekolah");
        assert_eq!(stemming.stem("menyapu di selokan"), "sapu selokan");
        assert_eq!(stemming.stem("pemusnahan sampah"), "musnah sampah");
        assert_eq!(stemming.stem("pemrograman"), "program");
    }
}
