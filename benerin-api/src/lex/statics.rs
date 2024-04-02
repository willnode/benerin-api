use std::collections::HashMap;
use std::collections::HashSet;

pub static EMPTY_STR: &'static str = "";

lazy_static! {
    pub static ref PUNCTUATIONS: HashSet<char> = ".,;:?!\n".chars().collect::<HashSet<char>>();
    pub static ref KIND_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("", ""),
        ("a", "a"),
        ("adv", "adv"),
        ("n", "n"),
        ("num", "num"),
        ("p", "p"),
        ("pron", "pron"),
        ("v", "v"),
        ("infiks", "infiks"),
        ("prefiks", "prefiks"),
        ("sufiks", "sufiks"),
    ]);
}
