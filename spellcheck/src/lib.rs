use symspell::{UnicodeStringStrategy, SymSpell, Suggestion, Verbosity};

extern crate strsim;
#[macro_use]
extern crate derive_builder;

pub mod symspell;

pub struct SpellCheck {
    symspell: SymSpell<UnicodeStringStrategy>,
    verbosity: Verbosity,
    max_edit_distance: i64,
}

impl SpellCheck {
    pub fn new() -> SpellCheck {
        let mut symspell: SymSpell<UnicodeStringStrategy> = SymSpell::default();
        let corpus = benerin_data::get_gram_words_in_csv_str();
        symspell.load_dictionary(&corpus, 0, 1, ",");
        SpellCheck {
            symspell,
            verbosity: Verbosity::Top,
            max_edit_distance: 2
        }
    }

    pub fn lookup_word(&self, input: &str) -> Vec<Suggestion> {
        self.symspell.lookup(input, self.verbosity, self.max_edit_distance)
    }

    pub fn lookup(&self, input: &str) -> Vec<Suggestion> {
        self.symspell.lookup_compound(input, self.max_edit_distance)
    }
}


#[test]
fn it_works() {
    let spellcheck = SpellCheck::new();
    assert_eq!(spellcheck.lookup_word("kvcing"), vec![Suggestion::new("kucing", 1, 21)]);
    assert_eq!(spellcheck.lookup("kvcing lir"), vec![Suggestion::new("kucing air", 2, 0)]);
}
