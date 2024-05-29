use deepsize::DeepSizeOf;
use graph::Graph;
use symspell::{Suggestion, SymSpell, Verbosity};
use tokenizer::Tokenizer;

extern crate strsim;
#[macro_use]
extern crate derive_builder;

pub mod symspell;

#[derive(DeepSizeOf)]
pub struct SpellCheck {
    tokenizer: Tokenizer,
    symspell: SymSpell,
    verbosity: Verbosity,
    max_edit_distance: i32,
}

impl SpellCheck {
    pub fn new() -> SpellCheck {
        let mut symspell: SymSpell = SymSpell::default();
        let corpus = benerin_data::get_gram_words_in_csv_str();
        symspell.load_dictionary(&corpus, 0, 1, ",");
        SpellCheck {
            tokenizer: Tokenizer::new(),
            symspell,
            verbosity: Verbosity::Top,
            max_edit_distance: 2,
        }
    }

    pub fn debug_heap(&self) {
        self.symspell.debug_heap()
    }

    pub fn lookup_word(&self, input: &str) -> Vec<Suggestion> {
        self.symspell
            .lookup(input, self.verbosity, self.max_edit_distance)
    }

    pub fn lookup(&self, text: &str) -> String {
        let graph = self.tokenizer.parse(text.to_owned());
        let result = self.lookup_graph(&graph);
        self.tokenizer.render(&result)
    }

    pub fn lookup_graph(&self, graph: &Graph) -> Graph {
        let mut g = self
            .symspell
            .lookup_compound(&graph, self.max_edit_distance);
        g.trim_end();
        g
    }
}

#[test]
fn it_works() {
    let spellcheck = SpellCheck::new();
    assert_eq!(
        spellcheck.lookup_word("kvcing"),
        vec![Suggestion::new("kucing", 1, 21)]
    );
    assert_eq!(spellcheck.lookup("kvcing lir"), "kucing air");
}
