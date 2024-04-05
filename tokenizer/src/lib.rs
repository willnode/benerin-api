use std::collections::HashSet;

use graph::Graph;
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
            Ok(graph) => Ok(graph),
            Err(e) => Err(e),
        }
    }

    pub fn to_json(&self, graph: &Graph) -> Result<String, Error> {
        serde_json::to_string(&graph.lexicons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tokenizer = Tokenizer::new();
        let s = " Halo, apa kabar?. Rumah  baik ya! ";
        assert_eq!(s, tokenizer.render(&tokenizer.parse(s.to_owned())));
        let sj = "Halo apa kabar Rumah baik ya";
        assert_eq!(sj, tokenizer.render_flat(&tokenizer.parse(s.to_owned())));
        let sj = r#"[{"lexemes":[{"offset":1,"length":4,"suffix":0}],"offset":0,"prefix":1,"length":4,"suffix":1},{"lexemes":[{"offset":7,"length":3,"suffix":1},{"offset":11,"length":5,"suffix":0}],"offset":6,"prefix":1,"length":9,"suffix":1},{"lexemes":[],"offset":17,"prefix":0,"length":0,"suffix":1},{"lexemes":[{"offset":19,"length":5,"suffix":2},{"offset":26,"length":4,"suffix":1},{"offset":31,"length":2,"suffix":0}],"offset":18,"prefix":1,"length":14,"suffix":1},{"lexemes":[],"offset":34,"prefix":1,"length":0,"suffix":0}]"#;
        assert_eq!(sj, tokenizer.to_json(&tokenizer.parse(s.to_owned())).unwrap());
        let sj = r#"[]"#;
        assert_eq!(sj, tokenizer.to_json(&tokenizer.parse("".to_owned())).unwrap());
        let sj = r#"[{"lexemes":[],"offset":0,"prefix":1,"length":0,"suffix":0}]"#;
        assert_eq!(sj, tokenizer.to_json(&tokenizer.parse(" ".to_owned())).unwrap());
    }
}
