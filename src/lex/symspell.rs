use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::i64;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::lex::statics::EMPTY_STR;

use super::edit_distance::{DistanceAlgorithm, EditDistance};
use super::string_strategy::StringStrategy;
use super::suggestion::Correction;
use super::suggestion::Lexicon;
use super::suggestion::Suggestion;

#[derive(Eq, PartialEq, Debug)]
pub enum Verbosity {
    Top,
    #[allow(dead_code)]
    Closest,
    All,
}

#[derive(Builder, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymSpell<T: StringStrategy> {
    /// Maximum edit distance for doing lookups.
    #[builder(default = "2")]
    max_dictionary_edit_distance: i64,
    /// The length of word prefixes used for spell checking.
    #[builder(default = "7")]
    prefix_length: i64,
    /// The minimum frequency count for dictionary words to be considered correct spellings.
    #[builder(default = "1")]
    count_threshold: i64,

    //// number of all words in the corpus used to generate the
    //// frequency dictionary. This is used to calculate the word
    //// occurrence probability p from word counts c : p=c/N. N equals
    //// the sum of all counts c in the dictionary only if the
    //// dictionary is complete, but not if the dictionary is
    //// truncated or filtered
    #[builder(default = "1_024_908_267_229", setter(skip))]
    corpus_word_count: i64,

    #[builder(default = "0", setter(skip))]
    max_length: i64,
    #[builder(default = "HashMap::new()", setter(skip))]
    deletes: HashMap<u64, Vec<Box<str>>>,
    #[builder(default = "HashMap::new()", setter(skip))]
    words: HashMap<Box<str>, i64>,
    #[builder(default = "HashMap::new()", setter(skip))]
    bigrams: HashMap<Box<str>, i64>,
    #[builder(default = "i64::MAX", setter(skip))]
    bigram_min_count: i64,
    #[builder(default = "DistanceAlgorithm::Damerau")]
    distance_algorithm: DistanceAlgorithm,
    #[builder(default = "T::new()", setter(skip))]
    string_strategy: T,
}

impl<T: StringStrategy> Default for SymSpell<T> {
    fn default() -> SymSpell<T> {
        SymSpellBuilder::default().build().unwrap()
    }
}

impl<T: StringStrategy> SymSpell<T> {
    /// Load multiple dictionary entries from a file of word/frequency count pairs.
    ///
    /// # Arguments
    ///
    /// * `corpus` - The path+filename of the file.
    /// * `term_index` - The column position of the word.
    /// * `count_index` - The column position of the frequency count.
    /// * `separator` - Separator between word and frequency
    pub fn load_dictionary(
        &mut self,
        corpus: &str,
        term_index: i64,
        count_index: i64,
        separator: &str,
    ) -> bool {
        if !Path::new(corpus).exists() {
            return false;
        }

        let file = File::open(corpus).expect("file not found");
        let sr = BufReader::new(file);

        for (i, line) in sr.lines().enumerate() {
            if i % 50_000 == 0 {
                eprintln!("progress: {}", i);
            }
            let line_str = line.unwrap();
            self.load_dictionary_line(&line_str, term_index, count_index, separator);
        }
        true
    }

    /// Load single dictionary entry from word/frequency count pair.
    ///
    /// # Arguments
    ///
    /// * `line` - word/frequency pair.
    /// * `term_index` - The column position of the word.
    /// * `count_index` - The column position of the frequency count.
    /// * `separator` - Separator between word and frequency
    pub fn load_dictionary_line(
        &mut self,
        line: &str,
        term_index: i64,
        count_index: i64,
        separator: &str,
    ) -> bool {
        let line_parts: Vec<&str> = line.split(separator).collect();
        if line_parts.len() >= 2 {
            // let key = unidecode(line_parts[term_index as usize]);
            let key = self
                .string_strategy
                .prepare(line_parts[term_index as usize]);
            let count = line_parts[count_index as usize].parse::<i64>().unwrap();

            self.create_dictionary_entry(key, count);
        }
        true
    }

    pub fn lookup(
        &self,
        input: &str,
        verbosity: Verbosity,
        max_edit_distance: i64,
    ) -> Vec<Suggestion> {
        if max_edit_distance > self.max_dictionary_edit_distance {
            panic!("max_edit_distance is bigger than max_dictionary_edit_distance");
        }

        let mut suggestions: Vec<Suggestion> = Vec::new();

        let prep_input = self.string_strategy.prepare(input);
        let input = prep_input.as_str();
        let input_len = self.string_strategy.len(input) as i64;

        if input_len - self.max_dictionary_edit_distance > self.max_length {
            return suggestions;
        }

        let mut hashset1: HashSet<String> = HashSet::new();
        let mut hashset2: HashSet<String> = HashSet::new();

        if self.words.contains_key(input) {
            let suggestion_count = self.words[input];
            suggestions.push(Suggestion::new(input, 0, suggestion_count));

            if verbosity != Verbosity::All {
                return suggestions;
            }
        }

        hashset2.insert(input.to_string());

        let mut max_edit_distance2 = max_edit_distance;
        let mut candidate_pointer = 0;
        let mut candidates = Vec::new();

        let mut input_prefix_len = input_len;

        if input_prefix_len > self.prefix_length {
            input_prefix_len = self.prefix_length;
            candidates.push(
                self.string_strategy
                    .slice(input, 0, input_prefix_len as usize),
            );
        } else {
            candidates.push(input.to_string());
        }

        let distance_comparer = EditDistance::new(self.distance_algorithm.clone());

        while candidate_pointer < candidates.len() {
            let candidate = &candidates.get(candidate_pointer).unwrap().clone();
            candidate_pointer += 1;
            let candidate_len = self.string_strategy.len(candidate) as i64;
            let length_diff = input_prefix_len - candidate_len;

            if length_diff > max_edit_distance2 {
                if verbosity == Verbosity::All {
                    continue;
                }
                break;
            }

            if self.deletes.contains_key(&self.get_string_hash(&candidate)) {
                let dict_suggestions = &self.deletes[&self.get_string_hash(&candidate)];

                for suggestion in dict_suggestions {
                    let suggestion_len = self.string_strategy.len(suggestion) as i64;

                    if suggestion.as_ref() == input {
                        continue;
                    }

                    if (suggestion_len - input_len).abs() > max_edit_distance2
                        || suggestion_len < candidate_len
                        || (suggestion_len == candidate_len && suggestion.as_ref() != candidate)
                    {
                        continue;
                    }

                    let sugg_prefix_len = cmp::min(suggestion_len, self.prefix_length);

                    if sugg_prefix_len > input_prefix_len
                        && sugg_prefix_len - candidate_len > max_edit_distance2
                    {
                        continue;
                    }

                    let distance;

                    if candidate_len == 0 {
                        distance = cmp::max(input_len, suggestion_len);

                        if distance > max_edit_distance2 || hashset2.contains(suggestion.as_ref()) {
                            continue;
                        }
                        hashset2.insert(suggestion.to_string());
                    } else if suggestion_len == 1 {
                        distance = if !input.contains(&self.string_strategy.slice(suggestion, 0, 1))
                        {
                            input_len
                        } else {
                            input_len - 1
                        };

                        if distance > max_edit_distance2 || hashset2.contains(suggestion.as_ref()) {
                            continue;
                        }

                        hashset2.insert(suggestion.to_string());
                    } else if self.has_different_suffix(
                        max_edit_distance,
                        input,
                        input_len,
                        candidate_len,
                        suggestion,
                        suggestion_len,
                    ) {
                        continue;
                    } else {
                        if verbosity != Verbosity::All
                            && !self.delete_in_suggestion_prefix(
                                candidate,
                                candidate_len,
                                suggestion,
                                suggestion_len,
                            )
                        {
                            continue;
                        }

                        if hashset2.contains(suggestion.as_ref()) {
                            continue;
                        }
                        hashset2.insert(suggestion.to_string());

                        distance = distance_comparer.compare(input, suggestion, max_edit_distance2);

                        if distance < 0 {
                            continue;
                        }
                    }

                    if distance <= max_edit_distance2 {
                        let suggestion_count = self.words[suggestion];
                        let si = Suggestion::new(suggestion.as_ref(), distance, suggestion_count);

                        if !suggestions.is_empty() {
                            match verbosity {
                                Verbosity::Closest => {
                                    if distance < max_edit_distance2 {
                                        suggestions.clear();
                                    }
                                }
                                Verbosity::Top => {
                                    if distance < max_edit_distance2
                                        || suggestion_count > suggestions[0].count
                                    {
                                        max_edit_distance2 = distance;
                                        suggestions[0] = si;
                                    }
                                    continue;
                                }
                                _ => (),
                            }
                        }

                        if verbosity != Verbosity::All {
                            max_edit_distance2 = distance;
                        }

                        suggestions.push(si);
                    }
                }
            }

            if length_diff < max_edit_distance && candidate_len <= self.prefix_length {
                if verbosity != Verbosity::All && length_diff >= max_edit_distance2 {
                    continue;
                }

                for i in 0..candidate_len {
                    let delete = self.string_strategy.remove(candidate, i as usize);

                    if !hashset1.contains(&delete) {
                        hashset1.insert(delete.clone());
                        candidates.push(delete);
                    }
                }
            }
        }

        if suggestions.len() > 1 {
            suggestions.sort();
        }

        suggestions
    }

    pub fn lookup_compound(&self, term_list: &mut Lexicon, edit_distance_max: i64) {
        //parse input string into single terms

        // let mut suggestions_previous_term: Vec<Suggestion> = Vec::new();                  //suggestions for a single term
        let mut suggestions: Vec<Suggestion>;
        //
        let distance_comparer = EditDistance::new(self.distance_algorithm.clone());

        //translate every term to its best suggestion, otherwise it remains unchanged
        let mut last_combi = false;

        for (i, term) in term_list.lexemes.iter().enumerate() {
            suggestions = self.lookup(term.word, Verbosity::Top, edit_distance_max);
            let mut suggestion_parts: Vec<Suggestion> = Vec::new();
            //combi check, always before split
            if i > 0 && !last_combi {
                let mut suggestions_combi: Vec<Suggestion> = self.lookup(
                    &format!(
                        "{}{}",
                        term_list.lexemes[i - 1].word,
                        term_list.lexemes[i].word
                    ),
                    Verbosity::Top,
                    edit_distance_max,
                );

                if !suggestions_combi.is_empty() {
                    let best1 = suggestion_parts[suggestion_parts.len() - 1].clone();
                    let best2 = if !suggestions.is_empty() {
                        suggestions[0].clone()
                    } else {
                        Suggestion::new(
                            term_list.lexemes[1].word,
                            edit_distance_max + 1,
                            10 / (10i64)
                                .pow(self.string_strategy.len(&term_list.lexemes[i].word) as u32),
                        )
                    };

                    //if (suggestions_combi[0].distance + 1 < DamerauLevenshteinDistance(term_list1[i - 1] + " " + term_list1[i], best1.term + " " + best2.term))
                    let distance1 = best1.distance + best2.distance;

                    if (distance1 >= 0)
                        && (suggestions_combi[0].distance + 1 < distance1
                            || (suggestions_combi[0].distance + 1 == distance1
                                && (suggestions_combi[0].count
                                    > best1.count / self.corpus_word_count * best2.count)))
                    {
                        suggestions_combi[0].distance += 1;
                        let last_i = suggestion_parts.len() - 1;
                        suggestion_parts[last_i] = suggestions_combi[0].clone();
                        last_combi = true;
                        continue;
                    }
                }
            }
            last_combi = false;

            //alway split terms without suggestion / never split terms with suggestion ed=0 / never split single char terms
            if !suggestions.is_empty()
                && ((suggestions[0].distance == 0)
                    || (self.string_strategy.len(&term_list.lexemes[i].word) == 1))
            {
                //choose best suggestion
                suggestion_parts.push(suggestions[0].clone());
            } else {
                let mut suggestion_split_best = if !suggestions.is_empty() {
                    //add original term
                    suggestions[0].clone()
                } else {
                    //if no perfect suggestion, split word into pairs
                    Suggestion::empty()
                };

                let term_length = self.string_strategy.len(&term_list.lexemes[i].word);

                if term_length > 1 {
                    for j in 1..term_length {
                        let part1 = self.string_strategy.slice(&term_list.lexemes[i].word, 0, j);
                        let part2 =
                            self.string_strategy
                                .slice(&term_list.lexemes[i].word, j, term_length);

                        let mut suggestion_split = Suggestion::empty();

                        let suggestions1 = self.lookup(&part1, Verbosity::Top, edit_distance_max);

                        if !suggestions1.is_empty() {
                            let suggestions2 =
                                self.lookup(&part2, Verbosity::Top, edit_distance_max);

                            if !suggestions2.is_empty() {
                                //select best suggestion for split pair
                                suggestion_split.term =
                                    format!("{} {}", suggestions1[0].term, suggestions2[0].term);

                                let mut distance2 = distance_comparer.compare(
                                    &term_list.lexemes[i].word,
                                    &format!("{} {}", suggestions1[0].term, suggestions2[0].term),
                                    edit_distance_max,
                                );

                                if distance2 < 0 {
                                    distance2 = edit_distance_max + 1;
                                }

                                if suggestion_split_best.term != EMPTY_STR {
                                    if distance2 > suggestion_split_best.distance {
                                        continue;
                                    }
                                    if distance2 < suggestion_split_best.distance {
                                        suggestion_split_best = Suggestion::empty();
                                    }
                                }
                                let count2: i64 = match self.bigrams.get(&*suggestion_split.term) {
                                    Some(&bigram_frequency) => {
                                        // increase count, if split
                                        // corrections are part of or
                                        // identical to input single term
                                        // correction exists
                                        if !suggestions.is_empty() {
                                            let best_si = &suggestions[0];
                                            // # alternatively remove the
                                            // # single term from
                                            // # suggestion_split, but then
                                            // # other splittings could win
                                            if suggestion_split.term == term_list.lexemes[i].word {
                                                // # make count bigger than
                                                // # count of single term
                                                // # correction
                                                cmp::max(bigram_frequency, best_si.count + 2)
                                            } else if suggestions1[0].term == best_si.term
                                                || suggestions2[0].term == best_si.term
                                            {
                                                // # make count bigger than
                                                // # count of single term
                                                // # correction
                                                cmp::max(bigram_frequency, best_si.count + 1)
                                            } else {
                                                bigram_frequency
                                            }
                                        // no single term correction exists
                                        } else if suggestion_split.term == term_list.lexemes[i].word
                                        {
                                            cmp::max(
                                                bigram_frequency,
                                                cmp::max(
                                                    suggestions1[0].count,
                                                    suggestions2[0].count,
                                                ) + 2,
                                            )
                                        } else {
                                            bigram_frequency
                                        }
                                    }
                                    None => {
                                        // The Naive Bayes probability of
                                        // the word combination is the
                                        // product of the two word
                                        // probabilities: P(AB)=P(A)*P(B)
                                        // use it to estimate the frequency
                                        // count of the combination, which
                                        // then is used to rank/select the
                                        // best splitting variant
                                        cmp::min(
                                            self.bigram_min_count,
                                            ((suggestions1[0].count as f64)
                                                / (self.corpus_word_count as f64)
                                                * (suggestions2[0].count as f64))
                                                as i64,
                                        )
                                    }
                                };
                                suggestion_split.distance = distance2;
                                suggestion_split.count = count2;

                                //early termination of split
                                if suggestion_split_best.term == EMPTY_STR
                                    || suggestion_split.count > suggestion_split_best.count
                                {
                                    suggestion_split_best = suggestion_split.clone();
                                }
                            }
                        }
                    }

                    if suggestion_split_best.term != EMPTY_STR {
                        //select best suggestion for split pair
                        suggestion_parts.push(suggestion_split_best.clone());
                    } else {
                        let mut si = Suggestion::empty();
                        // NOTE: this effectively clamps si_count to a certain minimum value, which it can't go below
                        let si_count: f64 = 10f64
                            / ((10i64).saturating_pow(
                                self.string_strategy.len(&term_list.lexemes[i].word) as u32,
                            )) as f64;

                        si.term = term_list.lexemes[i].word.to_string();
                        si.count = si_count as i64;
                        si.distance = edit_distance_max + 1;
                        suggestion_parts.push(si);
                    }
                } else {
                    let mut si = Suggestion::empty();
                    // NOTE: this effectively clamps si_count to a certain minimum value, which it can't go below
                    let si_count: f64 = 10f64
                        / ((10i64).saturating_pow(
                            self.string_strategy.len(&term_list.lexemes[i].word) as u32,
                        )) as f64;

                    si.term = term_list.lexemes[i].word.to_string();
                    si.count = si_count as i64;
                    si.distance = edit_distance_max + 1;
                    suggestion_parts.push(si);
                }
            }
            if suggestion_parts.len() > 0 {
                term_list.corrections.push(Correction {
                    start_lexeme: i,
                    end_lexeme: i,
                    r#type: "misspelling",
                    suggestion: suggestion_parts,
                });
            }
        }
    }

    fn delete_in_suggestion_prefix(
        &self,
        delete: &str,
        delete_len: i64,
        suggestion: &str,
        suggestion_len: i64,
    ) -> bool {
        if delete_len == 0 {
            return true;
        }
        let suggestion_len = if self.prefix_length < suggestion_len {
            self.prefix_length
        } else {
            suggestion_len
        };
        let mut j = 0;
        for i in 0..delete_len {
            let del_char = self.string_strategy.at(delete, i as isize).unwrap();
            while j < suggestion_len
                && del_char != self.string_strategy.at(suggestion, j as isize).unwrap()
            {
                j += 1;
            }

            if j == suggestion_len {
                return false;
            }
        }
        true
    }

    fn create_dictionary_entry<K>(&mut self, key: K, count: i64) -> bool
    where
        K: Clone + AsRef<str> + Into<String>,
    {
        if count < self.count_threshold {
            return false;
        }

        let key_clone = key.clone().into().into_boxed_str();

        match self.words.get(key.as_ref()) {
            Some(i) => {
                let updated_count = if i64::MAX - i > count {
                    i + count
                } else {
                    i64::MAX
                };
                self.words.insert(key_clone, updated_count);
                return false;
            }
            None => {
                self.words.insert(key_clone, count);
            }
        }

        let key_len = self.string_strategy.len(key.as_ref());

        if key_len as i64 > self.max_length {
            self.max_length = key_len as i64;
        }

        let edits = self.edits_prefix(key.as_ref());

        for delete in edits {
            let delete_hash = self.get_string_hash(&delete);

            self.deletes
                .entry(delete_hash)
                .and_modify(|e| e.push(key.clone().into().into_boxed_str()))
                .or_insert_with(|| vec![key.clone().into().into_boxed_str()]);
        }

        true
    }

    fn edits_prefix(&self, key: &str) -> HashSet<String> {
        let mut hash_set = HashSet::new();

        let key_len = self.string_strategy.len(key) as i64;

        if key_len <= self.max_dictionary_edit_distance {
            hash_set.insert("".to_string());
        }

        if key_len > self.prefix_length {
            let shortened_key = self
                .string_strategy
                .slice(key, 0, self.prefix_length as usize);
            hash_set.insert(shortened_key.clone());
            self.edits(&shortened_key, 0, &mut hash_set);
        } else {
            hash_set.insert(key.to_string());
            self.edits(key, 0, &mut hash_set);
        };

        hash_set
    }

    fn edits(&self, word: &str, edit_distance: i64, delete_words: &mut HashSet<String>) {
        let edit_distance = edit_distance + 1;
        let word_len = self.string_strategy.len(word);

        if word_len > 1 {
            for i in 0..word_len {
                let delete = self.string_strategy.remove(word, i);

                if !delete_words.contains(&delete) {
                    delete_words.insert(delete.clone());

                    if edit_distance < self.max_dictionary_edit_distance {
                        self.edits(&delete, edit_distance, delete_words);
                    }
                }
            }
        }
    }

    fn has_different_suffix(
        &self,
        max_edit_distance: i64,
        input: &str,
        input_len: i64,
        candidate_len: i64,
        suggestion: &str,
        suggestion_len: i64,
    ) -> bool {
        // handles the shortcircuit of min_distance
        // assignment when first boolean expression
        // evaluates to false
        let min = if self.prefix_length - max_edit_distance == candidate_len {
            cmp::min(input_len, suggestion_len) - self.prefix_length
        } else {
            0
        };

        (self.prefix_length - max_edit_distance == candidate_len)
            && (((min - self.prefix_length) > 1)
                && (self
                    .string_strategy
                    .suffix(input, (input_len + 1 - min) as usize)
                    != self
                        .string_strategy
                        .suffix(suggestion, (suggestion_len + 1 - min) as usize)))
            || ((min > 0)
                && (self.string_strategy.at(input, (input_len - min) as isize)
                    != self
                        .string_strategy
                        .at(suggestion, (suggestion_len - min) as isize))
                && ((self
                    .string_strategy
                    .at(input, (input_len - min - 1) as isize)
                    != self
                        .string_strategy
                        .at(suggestion, (suggestion_len - min) as isize))
                    || (self.string_strategy.at(input, (input_len - min) as isize)
                        != self
                            .string_strategy
                            .at(suggestion, (suggestion_len - min - 1) as isize))))
    }

    fn get_string_hash(&self, s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }

}
