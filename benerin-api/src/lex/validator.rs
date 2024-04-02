use super::{suggestion::{Lexicon, Correction, Suggestion}};


pub fn chk_double_space(lexicon: &mut Lexicon) {
    for (i, lexeme) in &mut lexicon.lexemes.iter().enumerate() {
        // Check if suffix contain two or more spaces
        if lexeme.suffix != "" && lexeme.suffix.chars().filter(|c| c.is_whitespace()).count() > 1 {
            // Create suggestion with one space
            let mut suggestion = String::from(lexeme.word);
            suggestion.push_str(" ");

            lexicon.corrections.push(Correction {
                start_lexeme: i,
                end_lexeme: i,
                r#type: "extra-space",
                suggestion: Some(Suggestion::new(suggestion, 0, 0)),
            });
        }
    }
}