mod types;

impl Graph<'_> {
    pub fn new(input: &str) -> Graph {
        Graph {
            lexicons: vec![],
            input
        }
    }
}

pub use types::{Graph, Lexicon, Lexeme};
