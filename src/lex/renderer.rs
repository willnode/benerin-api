use super::Lexicon;

pub fn run_renderer(graph: &Vec<Lexicon>) -> String {
    let mut text = String::new();
    for lexicon in graph {
        text.push_str(&lexicon.prefix);
        'lex: for (i, lexeme) in lexicon.lexemes.iter().enumerate() {
            // check for corrections
            for correction in &lexicon.corrections {
                if correction.start_lexeme == i {
                    text.push_str("<u-x data-type=\"");
                    text.push_str(&correction.r#type);
                    text.push_str("\"");
                    if !correction.suggestion.is_none() {
                        text.push_str(" data-suggestion=\"");
                        text.push_str(&correction.suggestion.as_ref().unwrap());
                        text.push_str("\">");
                    } else {
                        text.push_str(">");
                    }
                    text.push_str(&lexeme.word);
                    text.push_str(&lexeme.suffix);
                    text.push_str("</u-x>");
                    continue 'lex;
                }
            }
            text.push_str(&lexeme.word);
            text.push_str(&lexeme.suffix);
        }
        text.push_str(&lexicon.suffix);
    }
    text
}
