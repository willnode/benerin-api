

pub mod composition;
pub mod edit_distance;
pub mod string_strategy;
pub mod suggestion;
pub mod symspell;

pub use string_strategy::UnicodeStringStrategy;
pub use suggestion::Suggestion;
pub use symspell::{SymSpell, SymSpellBuilder, Verbosity};

