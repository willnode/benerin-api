use std::collections::HashSet;

use benerin_data::EMPTY_STR;
use graph::{Graph, Lexicon};
use serde_json::Error;

mod parser;
mod renderer;

pub struct Tokenizer {
    punctuations: HashSet<char>,
}

impl Tokenizer {
    // Initialization function
    pub fn new() -> Self {
        Tokenizer {
            punctuations: benerin_data::get_punctuations_in_hash_set(),
        }
    }

    pub fn from_json<'a>(&'a self, data: &'a str) -> Result<Graph, Error> {
        match serde_json::from_str(data) {
            Ok(lexicons) => Ok(Graph {
                input: &EMPTY_STR,
                lexicons,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn to_json(&self, graph: &Graph) -> Result<String, Error> {
        serde_json::to_string(&graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tokenizer = Tokenizer::new();
        let s = " Halo, apa kabar?. Rumah  baik ya! ";
        assert_eq!(s, tokenizer.render(&tokenizer.parse(s)));
        let sj = "Halo apa kabar Rumah baik ya";
        assert_eq!(sj, tokenizer.render_flat(&tokenizer.parse(s)));
        let sj = r#"[{"lexemes":[{"word":"Halo"}],"prefix":" ","suffix":","},{"lexemes":[{"word":"apa","suffix":" "},{"word":"kabar"}],"prefix":" ","suffix":"?"},{"lexemes":[],"suffix":"."},{"lexemes":[{"word":"Rumah","suffix":"  "},{"word":"baik","suffix":" "},{"word":"ya"}],"prefix":" ","suffix":"!"},{"lexemes":[],"prefix":" "}]"#;
        assert_eq!(sj, tokenizer.to_json(&tokenizer.parse(s)).unwrap());
    }
}
