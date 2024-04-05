use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub text: String,
    pub lexicons: Vec<Lexicon>,
    pub using_keys: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lexicon {
    pub lexemes: Vec<Lexeme>,
    pub offset: usize,
    pub prefix: usize,
    pub length: usize,

    pub suffix: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lexeme {
    pub offset: usize,
    pub length: usize,
    pub suffix: usize,
    #[serde(skip_serializing_if = "metadata_is_empty")]
    pub metadata: LexemeMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LexemeMetadata {
    pub key: usize,
    #[serde(skip_serializing_if = "pos_is_empty")]
    pub pos: PosTagging,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum PosTagging {
    Unset,
    Unknown
}

fn metadata_is_empty(metadata: &LexemeMetadata) -> bool {
    metadata.key == 0
}

fn pos_is_empty(metadata: &PosTagging) -> bool {
     *metadata == PosTagging::Unset
}
