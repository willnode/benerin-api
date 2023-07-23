use super::suggestion::{Lexeme, Lexicon};
use super::statics::{PUNCTUATIONS, EMPTY_STR};


pub fn run_parser(text: &str) -> Vec<Lexicon> {
    let mut lexicons: Vec<Lexicon> = vec![];
    // loop through text
    let mut current_lexicon = Lexicon {
        lexemes: vec![],
        corrections: vec![],
        prefix: EMPTY_STR,
        suffix: EMPTY_STR,
    };
    // loop every character
    let mut indices = text.char_indices().peekable();
    while let Some((i, c)) = indices.next() {
        // check if character is punctuation
        if PUNCTUATIONS.contains(&c) {
            // end of lexicon
            current_lexicon.suffix = &text[i..i + 1];
            lexicons.push(current_lexicon);
            current_lexicon = Lexicon {
                lexemes: vec![],
                corrections: vec![],
                prefix: EMPTY_STR,
                suffix: EMPTY_STR,
            };
        }
        // check if character is whitespace
        else if c.is_whitespace() {
            // get until next character isn't whitespace
            let mut i2 = i + 1;
            while let Some((i, c)) = indices.peek() {
                i2 = *i;
                if !c.is_whitespace() || PUNCTUATIONS.contains(&c) {
                    break;
                } else {
                    indices.next();
                }
            }
            // start of lexicon?
            if current_lexicon.lexemes.len() == 0 {
                current_lexicon.prefix = &text[i..i2];
            }
            // put it to last lexeme
            else {
                current_lexicon.lexemes.last_mut().unwrap().suffix = &text[i..i2];
            }
        } else {
            // get until next character is whitespace or punctuation
            let mut i2 = i;
            while let Some((i, c)) = indices.peek() {
                if c.is_whitespace() || PUNCTUATIONS.contains(&c) {
                    break;
                } else {
                    i2 = *i + 1;
                    indices.next();
                }
            }
            // push lexeme
            let word = &text[i..i2];
            current_lexicon.lexemes.push(Lexeme {
                word,
                kind: EMPTY_STR,
                suffix: EMPTY_STR,
            });
        }
    }

    if current_lexicon.lexemes.len() > 0 || current_lexicon.prefix != EMPTY_STR {
        lexicons.push(current_lexicon);
    }

    lexicons
}
