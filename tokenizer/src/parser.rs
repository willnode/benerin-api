use graph::{Graph, Lexeme, Lexicon};

use crate::Tokenizer;

impl Tokenizer {
    pub fn parse<'a>(&'a self, text: String) -> Graph {
        let mut lexicons: Vec<Lexicon> = vec![];
        let mut g = Graph::new(text, false);
        // loop through text
        let mut current_lexicon = Lexicon::new(0);
        // loop every character
        let mut indices = g.text.char_indices().peekable();
        while let Some((i, c)) = indices.next() {
            // check if character is punctuation
            if self.punctuations.contains(&c) {
                // end of lexicon
                current_lexicon.set_suffix(i + 1);
                lexicons.push(current_lexicon);
                current_lexicon = Lexicon::new(i + 1);
            }
            // check if character is whitespace
            else if c.is_whitespace() {
                // get until next character isn't whitespace
                let mut i2 = i + 1;
                while let Some((i, c)) = indices.peek() {
                    i2 = *i;
                    if !c.is_whitespace() || self.punctuations.contains(&c) {
                        break;
                    } else {
                        indices.next();
                    }
                }
                // start of lexicon?
                if current_lexicon.lexemes.len() == 0 {
                    current_lexicon.set_prefix(i2);
                }
                // put it to last lexeme
                else {
                    current_lexicon.lexemes.last_mut().unwrap().set_suffix(i2);
                    current_lexicon.set_length(i2);
                }
            } else {
                // get until next character is whitespace or punctuation
                let mut i2 = i;
                while let Some((i, c)) = indices.peek() {
                    if c.is_whitespace() || self.punctuations.contains(&c) {
                        break;
                    } else {
                        i2 = *i + 1;
                        indices.next();
                    }
                }
                // push lexeme
                let mut lexeme = Lexeme::new(i);
                lexeme.set_length(i2);
                current_lexicon.set_length(i2);
                current_lexicon.lexemes.push(lexeme);
            }
        }

        if current_lexicon.lexemes.len() > 0 || current_lexicon.prefix > 0 {
            lexicons.push(current_lexicon);
        }

        g.lexicons = lexicons;
        g
    }
}
