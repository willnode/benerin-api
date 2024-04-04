

use std::collections::HashSet;

use serde_json::Error;
use types::Lexicon;

mod types;
mod parser;
mod renderer;

pub struct Tokenizer {
    punctuations: HashSet<char>
} 


impl Tokenizer {
    // Initialization function
    pub fn new() -> Self {
        Tokenizer {
            punctuations: benerin_data::get_punctuations_in_hash_set()
        }
    }
    
    pub fn from_json<'a>(&'a self, data: &'a str) -> Result<Vec<Lexicon>, Error> {
        serde_json::from_str(data)
    }

    pub fn to_json(&self, graph: &Vec<Lexicon>) -> Result<String, Error> {
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
        let sj = r#"[{"lexemes":[{"word":"Halo"}],"prefix":" ","suffix":","},{"lexemes":[{"word":"apa","suffix":" "},{"word":"kabar"}],"prefix":" ","suffix":"?"},{"lexemes":[],"suffix":"."},{"lexemes":[{"word":"Rumah","suffix":"  "},{"word":"baik","suffix":" "},{"word":"ya"}],"prefix":" ","suffix":"!"},{"lexemes":[],"prefix":" "}]"#;
        assert_eq!(sj, tokenizer.to_json(&tokenizer.parse(s)).unwrap());
    }
}
