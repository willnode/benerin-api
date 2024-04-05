use graph::{Graph, Lexeme, Lexicon};
use postemi::Postemi;
use sastrawi::Sastrawi;
use std::collections::HashSet;
use tokenizer::Tokenizer;

mod postemi;
mod sastrawi;

pub struct Stemming {
    tokenizer: Tokenizer,
    stop_words: HashSet<String>,
    engine: StemmingEngine,
    pub use_stop_words: bool,
}

enum StemmingEngine {
    Sastrawi(Sastrawi),
    Postemi(Postemi),
}

impl Stemming {
    // Initialization function
    pub fn new(engine: &str) -> Self {
        Stemming {
            tokenizer: Tokenizer::new(),
            stop_words: benerin_data::get_stop_words_in_hash_set(),
            engine: if engine == "sastrawi" {
                StemmingEngine::Sastrawi(Sastrawi::new())
            } else {
                StemmingEngine::Postemi(Postemi::new())
            },
            use_stop_words: true,
        }
    }

    pub fn stem_word(&self, word: &str) -> String {
        match &self.engine {
            StemmingEngine::Postemi(x) => x.stem_word(word).unwrap_or(word).to_owned(),
            StemmingEngine::Sastrawi(x) => x.stem_word(word),
        }
    }

    pub fn stem_word_op(&self, word: &str) -> Option<&str> {
        match &self.engine {
            StemmingEngine::Postemi(x) => x.stem_word(word),
            StemmingEngine::Sastrawi(_) => None, // unimplemented
        }
    }

    pub fn stem(&self, text: &str) -> String {
        let graph = self.tokenizer.parse(text.to_owned());
        let result = self.stem_graph(&graph);
        self.tokenizer.render(&result)
    }

    pub fn stem_graph(&self, graph: &Graph) -> Graph {
        let mut g = Graph::new("".to_owned());
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
                let mut w = Lexeme::new(g.text.len());
                g.text.push_str(s);
                if g.using_keys {
                    if ow == s {
                        w.set_key(lexeme.metadata.key)
                    } else {
                        w.init_key()
                    }
                }
                w.set_length(g.text.len());
                g.text.push_str(" ");
                w.set_suffix(g.text.len());
                p.lexemes.push(w);
                p.set_length(g.text.len());
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
        let stemming = Stemming::new("");
        assert_eq!(stemming.stem("menari di sekolahan"), "tari sekolah");
        assert_eq!(stemming.stem("menyapu di selokan"), "sapu selokan");
        assert_eq!(stemming.stem("pemusnahan sampah"), "musnah sampah");
        assert_eq!(stemming.stem("pemrograman"), "program");
    }
}
