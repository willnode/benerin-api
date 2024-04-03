use postemi::Postemi;
use sastrawi::Sastrawi;
use std::collections::HashSet;

mod postemi;
mod sastrawi;

pub struct Stemming {
    stop_words: HashSet<String>,
    engine: StemmingEngine,
    use_normalize: bool,
    use_stopword_removal: bool,
}

enum StemmingEngine {
    Sastrawi(Sastrawi),
    Postemi(Postemi),
}

// pub trait StemmingEngine: Sized {
//     // Initialization function
//     fn new() -> Self;
//     fn stem_word(&self, word: &str) -> String;
// }

impl Stemming {
    // Initialization function
    pub fn new(engine: &str) -> Self {
        Stemming {
            stop_words: benerin_data::get_stop_words_in_hash_set(),
            engine: if engine == "sastrawi" {
                StemmingEngine::Sastrawi(Sastrawi::new())
            } else {
                StemmingEngine::Postemi(Postemi::new())
            },
            use_normalize: true,
            use_stopword_removal: true,
        }
    }

    fn normalize_text(&self, text: String) -> String {
        let mut processed_text = text.to_lowercase();
        processed_text = regex::Regex::new(r"[^a-z0-9 -]")
            .unwrap()
            .replace_all(&processed_text, " ")
            .to_string();
        processed_text = regex::Regex::new(r"( +)")
            .unwrap()
            .replace_all(&processed_text, " ")
            .to_string();

        processed_text.trim().to_owned()
    }

    pub fn stem_word(&self, word: &str) -> String {
        match &self.engine {
            StemmingEngine::Postemi(x) => x.stem_word(word),
            StemmingEngine::Sastrawi(x) => x.stem_word(word)
        }
    }

    pub fn stem(&self, text: &str) -> String {
        let mut text = text.to_owned();
        if self.use_normalize {
            text = self.normalize_text(text);
        }
        let tokens = if self.use_stopword_removal {
            text.split(" ")
                .map(|t| self.stem_word(t))
                .filter(|t| !self.stop_words.contains(t))
                .collect::<Vec<String>>()
        } else {
            text.split(" ")
                .map(|t| self.stem_word(t))
                .collect::<Vec<String>>()
        };
        tokens.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let stemming = Stemming::new("");
        // assert_eq!(stemming.stem("menari di sekolahan"), "tari sekolah");
        // assert_eq!(stemming.stem("menyapu di selokan"), "sapu selokan");
        assert_eq!(stemming.stem("pemusnahan sampah"), "musnah sampah");
        // assert_eq!(stemming.stem("pemrograman"), "program");
    }
}
