extern crate strsim;

mod edit_distance;
mod string_strategy;
mod symspell;

pub mod parser;
pub mod renderer;
pub mod statics;
pub mod suggestion;
pub mod validator;

pub use string_strategy::{StringStrategy, UnicodeStringStrategy};
pub use symspell::{SymSpell, SymSpellBuilder, Verbosity};

use self::{
    parser::run_parser, renderer::run_renderer, suggestion::Task, validator::chk_double_space,
};

lazy_static! {
    pub static ref SPELLENGINE: SymSpell<UnicodeStringStrategy> = init_spell_engine();
}

fn init_spell_engine() -> SymSpell<UnicodeStringStrategy> {
    let mut spellengine = SymSpell::default();
    spellengine.load_dictionary("data/gram.txt", 0, 1, "\t");
    // spellengine.load_bigram_dictionary("./data/bigram.txt", 0, 2, " ");
    println!("Spell engine loaded");
    spellengine
}

pub fn transform<'a>(text: &'a str) -> Task<'a> {
    let mut word_obj = run_parser(text);
    for lexicon in &mut word_obj {
        chk_double_space(lexicon);
        SPELLENGINE.lookup_compound(lexicon, 2);
    }
    let word = run_renderer(&word_obj);
    Task {
        status: "ok".to_string(),
        structure: word_obj,
        rendered: word,
    }
}
