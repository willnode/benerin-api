use super::{Correction, Lexicon};

fn chk_unk_words(lexicon: &mut Lexicon) {
    for (i, lexeme) in &mut lexicon.lexemes.iter().enumerate() {
        if lexeme.kind == "" {
            // Check if begin with capital letter
            if lexeme.word.chars().nth(0).unwrap().is_uppercase() {
                // names
                continue;
            }
            // Create suggestion with capital letter
            let mut suggestion = String::from(lexeme.word);
            suggestion.replace_range(0..1, &lexeme.word[0..1].to_uppercase());
            suggestion.push_str(lexeme.suffix);

            lexicon.corrections.push(Correction {
                start_lexeme: i,
                end_lexeme: i,
                r#type: "unknown-word",
                suggestion: Some(suggestion),
            });
        }
    }
}

fn chk_double_space(lexicon: &mut Lexicon) {
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
                suggestion: Some(suggestion),
            });
        }
    }
}

pub fn run_validator(graph: &mut Vec<Lexicon>) {
    for lexicon in graph {
        chk_unk_words(lexicon);
        chk_double_space(lexicon);
    }
}
