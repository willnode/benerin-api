use serde::{Serialize};
use std::cmp::Ordering;

#[derive(Serialize, Debug)]
pub struct Task<'a> {
    pub status: String,
    pub structure: Vec<Lexicon<'a>>,
    pub rendered: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Suggestion {
    pub term: String,
    pub distance: i64,
    pub count: i64,
}

#[derive(Serialize, Debug)]
pub struct Lexicon<'a> {
    pub lexemes: Vec<Lexeme<'a>>,
    pub corrections: Vec<Correction<'a>>,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub prefix: &'a str,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub suffix: &'a str,
}

#[derive(Serialize, Debug)]
pub struct Lexeme<'a> {
    pub word: &'a str,
    pub kind: &'a str,
    pub suffix: &'a str,
}

#[derive(Serialize, Debug)]
pub struct Correction<'a> {
    pub start_lexeme: usize,
    pub end_lexeme: usize,
    pub r#type: &'a str,
    pub suggestion: Vec<Suggestion>,
}

trait CorrectionExt {
    fn add_offset(&mut self, offset: usize);
}

impl CorrectionExt for Correction<'_> {
    fn add_offset(&mut self, offset: usize) {
        self.start_lexeme += offset;
        self.end_lexeme += offset;
    }
}

impl Suggestion {
    pub fn empty() -> Suggestion {
        Suggestion {
            term: "".to_string(),
            distance: 0,
            count: 0,
        }
    }

    pub fn new(term: impl Into<String>, distance: i64, count: i64) -> Suggestion {
        Suggestion {
            term: term.into(),
            distance,
            count,
        }
    }
}

impl Ord for Suggestion {
    fn cmp(&self, other: &Suggestion) -> Ordering {
        let distance_cmp = self.distance.cmp(&other.distance);
        if distance_cmp == Ordering::Equal {
            return self.count.cmp(&other.count);
        }
        distance_cmp
    }
}

impl PartialOrd for Suggestion {
    fn partial_cmp(&self, other: &Suggestion) -> Option<Ordering> {
        let distance_cmp = self.distance.cmp(&other.distance);
        if distance_cmp == Ordering::Equal {
            return Some(self.count.cmp(&other.count));
        }
        Some(distance_cmp)
    }
}

impl PartialEq for Suggestion {
    fn eq(&self, other: &Suggestion) -> bool {
        // self.term == other.term
        self.distance == other.distance && self.count == other.count
    }
}
impl Eq for Suggestion {}

fn str_is_empty<'a>(metadata: &'a str) -> bool {
    match metadata {
        "" => true,
        _ => false,
    }
}
