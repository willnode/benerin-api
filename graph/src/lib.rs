mod types;
use rand::Rng;
use trim_in_place::TrimInPlace;
pub use types::{Graph, Lexeme, LexemeMetadata, Lexicon, PosTagging};

impl Graph {
    pub fn new(text: String, using_keys: bool) -> Graph {
        Graph {
            lexicons: vec![],
            text,
            using_keys,
        }
    }

    pub fn push_word(&mut self, word: &str, existing_key: Option<(&str, usize)>) -> Lexeme {
        let mut l = Lexeme::new(self.text.len());
        self.text.push_str(word);
        l.set_length(self.text.len());
        if self.using_keys {
            match existing_key {
                Some((s, k)) => {
                    if s == word {
                        l.set_key(k)
                    } else {
                        l.init_key()
                    }
                }
                None => l.init_key(),
            }
        }
        l
    }

    pub fn get_key(&self, lex: &Lexeme) -> Option<(&str, usize)> {
        if self.using_keys {
            Some((self.get_word(lex), lex.metadata.key))
        } else {
            None
        }
    }

    pub fn push_str(&mut self, text: &str) -> usize {
        self.text.push_str(text);
        self.text.len()
    }

    pub fn get_word(&self, lex: &Lexeme) -> &str {
        if lex.offset + lex.length <= self.text.len() {
            &self.text[lex.offset..lex.offset + lex.length]
        } else {
            ""
        }
    }

    pub fn get_lexeme_suffix(&self, lex: &Lexeme) -> &str {
        let start = lex.offset + lex.length;
        if start + lex.suffix <= self.text.len() {
            &self.text[start..start + lex.suffix]
        } else {
            ""
        }
    }

    pub fn get_lexicon_suffix(&self, lex: &Lexicon) -> &str {
        let start = lex.offset + lex.prefix + lex.length;
        if start + lex.suffix <= self.text.len() {
            &self.text[start..start + lex.suffix]
        } else {
            ""
        }
    }

    pub fn get_lexicon_prefix(&self, lex: &Lexicon) -> &str {
        if lex.offset + lex.prefix <= self.text.len() {
            &self.text[lex.offset..lex.offset + lex.prefix]
        } else {
            ""
        }
    }

    pub fn init_hash_keys(&mut self) {
        let mut rng = rand::thread_rng();
        for lex in self.lexicons.iter_mut() {
            for le in lex.lexemes.iter_mut() {
                if le.metadata.key == 0 {
                    le.metadata.key = rng.gen();
                }
            }
        }
        self.using_keys = true
    }

    pub fn strip_hash_keys(&mut self) {
        for lex in self.lexicons.iter_mut() {
            for le in lex.lexemes.iter_mut() {
                le.metadata.key = 0;
            }
        }
        self.using_keys = false
    }

    pub fn trim_end(&mut self) {
        if self.lexicons.len() == 0 {
            return;
        }
        let lexicon = self.lexicons.last_mut().unwrap();
        if lexicon.lexemes.len() == 0 {
            return;
        }
        let lexeme = lexicon.lexemes.last_mut().unwrap();
        let old_len = self.text.len();
        let new_len = self.text.trim_end_in_place().len();
        if new_len < old_len {
            lexeme.set_suffix(new_len);
            lexicon.set_length(new_len);
            assert_eq!(lexicon.suffix, 0);
        }
    }
}

impl Lexicon {
    pub fn new(offset: usize) -> Lexicon {
        Lexicon {
            lexemes: vec![],
            offset,
            length: 0,
            prefix: 0,
            suffix: 0,
        }
    }
    pub fn set_suffix(&mut self, pos: usize) {
        self.suffix = pos - self.length - self.prefix - self.offset
    }
    pub fn set_length(&mut self, pos: usize) {
        self.length = pos - self.prefix - self.offset
    }
    pub fn set_prefix(&mut self, pos: usize) {
        self.prefix = pos - self.offset
    }
    pub fn push_lexeme(&mut self, lexeme: Lexeme) {
        self.set_length(lexeme.offset + lexeme.length + lexeme.suffix);
        self.lexemes.push(lexeme);
    }
}

impl Lexeme {
    pub fn new(offset: usize) -> Lexeme {
        Lexeme {
            offset,
            length: 0,
            suffix: 0,
            metadata: LexemeMetadata::default(),
        }
    }
    pub fn set_suffix(&mut self, pos: usize) {
        self.suffix = pos - self.length - self.offset
    }
    pub fn set_length(&mut self, pos: usize) {
        self.length = pos - self.offset
    }
    pub fn set_key(&mut self, key: usize) {
        self.metadata.key = key
    }
    pub fn init_key(&mut self) {
        let mut rng = rand::thread_rng();
        self.metadata.key = rng.gen();
    }
}

impl LexemeMetadata {
    pub fn default() -> LexemeMetadata {
        LexemeMetadata {
            key: 0,
            pos: PosTagging::Unset,
        }
    }
}
