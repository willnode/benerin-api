use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::i32;
use std::io::{BufRead, BufReader};
use std::path::Path;
use xxhash_rust::xxh64;
use deepsize::DeepSizeOf;
use graph::Graph;
use graph::Lexicon;

use super::composition::Composition;
use super::edit_distance::{DistanceAlgorithm, EditDistance};
use super::suggestion;
use super::suggestion::Suggestion;
use super::UnicodeStringStrategy;

#[derive(Eq, PartialEq, Debug, Clone, Copy, DeepSizeOf)]
pub enum Verbosity {
    Top,
    Closest,
    All,
}

#[derive(Builder, PartialEq, DeepSizeOf)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SymSpell {
    /// Maximum edit distance for doing lookups.
    #[builder(default = "2")]
    max_dictionary_edit_distance: i32,
    /// The length of word prefixes used for spell checking.
    #[builder(default = "7")]
    prefix_length: i32,
    /// The minimum frequency count for dictionary words to be considered correct spellings.
    #[builder(default = "10")]
    count_threshold: i32,

    //// number of all words in the corpus used to generate the
    //// frequency dictionary. This is used to calculate the word
    //// occurrence probability p from word counts c : p=c/N. N equals
    //// the sum of all counts c in the dictionary only if the
    //// dictionary is complete, but not if the dictionary is
    //// truncated or filtered
    #[builder(default = "1_024_908_267_229", setter(skip))]
    corpus_word_count: i32,

    #[builder(default = "0", setter(skip))]
    max_length: i32,
    #[builder(default = "HashMap::new()", setter(skip))]
    deletes: HashMap<u64, Vec<u32>>,
    #[builder(default = "HashMap::new()", setter(skip))]
    idword: HashMap<u32, Box<str>>,
    #[builder(default = "HashMap::new()", setter(skip))]
    wordid: HashMap<Box<str>, u32>,
    #[builder(default = "HashMap::new()", setter(skip))]
    wordfreq: HashMap<Box<str>, i32>,
    #[builder(default = "HashMap::new()", setter(skip))]
    bigrams: HashMap<Box<str>, i32>,
    #[builder(default = "i32::MAX", setter(skip))]
    bigram_min_count: i32,
    #[builder(default = "DistanceAlgorithm::Damerau")]
    distance_algorithm: DistanceAlgorithm,
    #[builder(default = "UnicodeStringStrategy::new()", setter(skip))]
    string_strategy: UnicodeStringStrategy,
}

impl Default for SymSpell {
    fn default() -> SymSpell {
        SymSpellBuilder::default().build().unwrap()
    }
}

impl SymSpell {
    /// Load multiple dictionary entries from a file of word/frequency count pairs.
    ///
    /// # Arguments
    ///
    /// * `corpus` - The content of the file.
    /// * `term_index` - The column position of the word.
    /// * `count_index` - The column position of the frequency count.
    /// * `separator` - Separator between word and frequency
    pub fn load_dictionary(
        &mut self,
        corpus: &str,
        term_index: i32,
        count_index: i32,
        separator: &str,
    ) {
        for line in corpus.lines() {
            self.load_dictionary_line(&line, term_index, count_index, separator);
        }
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
        term_index: i32,
        count_index: i32,
        separator: &str,
    ) {
        let line_parts: Vec<&str> = line.split(separator).collect();
        if line_parts.len() >= 2 {
            let key = self
                .string_strategy
                .prepare(line_parts[term_index as usize]);
            let count = line_parts[count_index as usize].parse::<i32>().unwrap();

            self.create_dictionary_entry(key, count);
        }
    }

    pub fn debug_heap(&self) {
        println!(
            "  bigrams heap: {:.2?} MB",
            self.bigrams.deep_size_of() / 1024 / 1024
        );
        println!(
            "  words heap: {:.2?} MB",
            self.wordfreq.deep_size_of() / 1024 / 1024
        );
        println!(
            "  deletes heap: {:.2?} MB",
            self.deletes.deep_size_of() / 1024 / 1024
        );
    }

    /// Load multiple bigram entries from a file of bigram/frequency count pairs.
    ///
    /// # Arguments
    ///
    /// * `corpus` - The path+filename of the file.
    /// * `term_index` - The column position of the word.
    /// * `count_index` - The column position of the frequency count.
    /// * `separator` - Separator between word and frequency
    pub fn load_bigram_dictionary(
        &mut self,
        corpus: &str,
        term_index: i32,
        count_index: i32,
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
            self.load_bigram_dictionary_line(&line_str, term_index, count_index, &separator);
        }
        true
    }

    /// Load single dictionary entry from bigram/frequency count pair.
    ///
    /// # Arguments
    ///
    /// * `line` - bigram/frequency pair.
    /// * `term_index` - The column position of the word.
    /// * `count_index` - The column position of the frequency count.
    /// * `separator` - Separator between word and frequency
    pub fn load_bigram_dictionary_line(
        &mut self,
        line: &str,
        term_index: i32,
        count_index: i32,
        separator: &str,
    ) -> bool {
        let line_parts: Vec<&str> = line.split(separator).collect();
        let line_parts_len = if separator == " " { 3 } else { 2 };
        if line_parts.len() >= line_parts_len {
            let key = if separator == " " {
                self.string_strategy.prepare(&format!(
                    "{} {}",
                    line_parts[term_index as usize],
                    line_parts[(term_index + 1) as usize]
                ))
            } else {
                self.string_strategy
                    .prepare(line_parts[term_index as usize])
            };
            let count = line_parts[count_index as usize].parse::<i32>().unwrap();
            self.bigrams.insert(key.into_boxed_str(), count);
            if count < self.bigram_min_count {
                self.bigram_min_count = count;
            }
        }
        true
    }

    /// Find suggested spellings for a given input word, using the maximum
    /// edit distance specified during construction of the SymSpell dictionary.
    ///
    /// # Arguments
    ///
    /// * `input` - The word being spell checked.
    /// * `verbosity` - The value controlling the quantity/closeness of the retuned suggestions.
    /// * `max_edit_distance` - The maximum edit distance between input and suggested words.
    ///
    /// # Examples
    ///
    /// ```
    /// use spellcheck::symspell::{SymSpell, Verbosity};
    ///
    /// let mut symspell: SymSpell = SymSpell::default();
    /// symspell.load_dictionary("whatever,2", 0, 1, " ");
    /// let r = symspell.lookup("whatver", Verbosity::Top, 2);
    /// ```
    pub fn lookup(
        &self,
        input: &str,
        verbosity: Verbosity,
        max_edit_distance: i32,
    ) -> Vec<Suggestion> {
        if max_edit_distance > self.max_dictionary_edit_distance {
            panic!("max_edit_distance is bigger than max_dictionary_edit_distance");
        }

        let mut suggestions: Vec<Suggestion> = Vec::new();

        let prep_input = self.string_strategy.prepare(input);
        let input = prep_input.as_str();
        let input_len = self.string_strategy.len(input) as i32;

        if input_len - self.max_dictionary_edit_distance > self.max_length {
            return suggestions;
        }

        let mut hashset1: HashSet<String> = HashSet::new();
        let mut hashset2: HashSet<String> = HashSet::new();

        if self.wordfreq.contains_key(input) {
            let suggestion_count = self.wordfreq[input];
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
            let candidate_len = self.string_strategy.len(candidate) as i32;
            let length_diff = input_prefix_len - candidate_len;

            if length_diff > max_edit_distance2 {
                if verbosity == Verbosity::All {
                    continue;
                }
                break;
            }

            if self.deletes.contains_key(&self.get_string_hash(&candidate)) {
                let dict_suggestions = &self.deletes[&self.get_string_hash(&candidate)];

                for suggestion_id in dict_suggestions {
                    let suggestion = self.idword[suggestion_id].as_ref();
                    let suggestion_len = self.string_strategy.len(suggestion) as i32;

                    if suggestion == input {
                        continue;
                    }

                    if (suggestion_len - input_len).abs() > max_edit_distance2
                        || suggestion_len < candidate_len
                        || (suggestion_len == candidate_len && suggestion != candidate)
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

                        if distance > max_edit_distance2 || hashset2.contains(suggestion) {
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

                        if distance > max_edit_distance2 || hashset2.contains(suggestion) {
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

                        if hashset2.contains(suggestion) {
                            continue;
                        }
                        hashset2.insert(suggestion.to_string());

                        distance = distance_comparer.compare(input, suggestion, max_edit_distance2);

                        if distance < 0 {
                            continue;
                        }
                    }

                    if distance <= max_edit_distance2 {
                        let suggestion_count = self.wordfreq[suggestion];
                        let si = Suggestion::new(suggestion, distance, suggestion_count);

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

    /// Find suggested spellings for a given input sentence, using the maximum
    /// edit distance specified during construction of the SymSpell dictionary.
    ///
    /// # Arguments
    ///
    /// * `input` - The sentence being spell checked.
    /// * `max_edit_distance` - The maximum edit distance between input and suggested words.
    ///
    pub fn lookup_compound(&self, graph: &Graph, edit_distance_max: i32) -> Graph {
        let mut g = Graph::new("".to_owned(), graph.using_keys);
        for lexicon in &graph.lexicons {
            let mut p = Lexicon::new(g.text.len());
            self.lookup_compound_lexicon(graph, lexicon, &mut g, &mut p, edit_distance_max);
            g.lexicons.push(p)
        }
        g
    }

    fn lookup_compound_lexicon(
        &self,
        input: &Graph,
        text: &Lexicon,
        g: &mut Graph,
        p: &mut Lexicon,
        edit_distance_max: i32,
    ) {
        //parse input string into single terms
        let term_list1 = &text.lexemes;

        // let mut suggestions_previous_term: Vec<Suggestion> = Vec::new();                  //suggestions for a single term
        let mut suggestions: Vec<Suggestion>;
        let mut suggestion_parts: Vec<Suggestion> = Vec::new();
        let distance_comparer = EditDistance::new(self.distance_algorithm.clone());

        //translate every term to its best suggestion, otherwise it remains unchanged
        let mut last_combi = false;

        for (i, term) in term_list1.iter().enumerate() {
            suggestions = self.lookup(input.get_word(term), Verbosity::Top, edit_distance_max);

            //combi check, always before split
            if i > 0 && !last_combi {
                let mut suggestions_combi: Vec<Suggestion> = self.lookup(
                    &format!(
                        "{}{}",
                        input.get_word(&term_list1[i - 1]),
                        input.get_word(&term_list1[i])
                    ),
                    Verbosity::Top,
                    edit_distance_max,
                );

                if !suggestions_combi.is_empty() {
                    let best1 = suggestion_parts[suggestion_parts.len() - 1].clone();
                    let best2 = if !suggestions.is_empty() {
                        suggestions[0].clone()
                    } else {
                        Suggestion::new(input.get_word(&term_list1[1]), edit_distance_max + 1, 0)
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
                    || (self.string_strategy.len(input.get_word(&term_list1[i])) == 1))
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

                let term_length = self.string_strategy.len(input.get_word(&term_list1[i]));

                if term_length > 1 {
                    for j in 1..term_length {
                        let part1 =
                            self.string_strategy
                                .slice(input.get_word(&term_list1[i]), 0, j);
                        let part2 = self.string_strategy.slice(
                            input.get_word(&term_list1[i]),
                            j,
                            term_length,
                        );

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
                                    input.get_word(&term_list1[i]),
                                    &format!("{} {}", suggestions1[0].term, suggestions2[0].term),
                                    edit_distance_max,
                                );

                                if distance2 < 0 {
                                    distance2 = edit_distance_max + 1;
                                }

                                if suggestion_split_best.term != "" {
                                    if distance2 > suggestion_split_best.distance {
                                        continue;
                                    }
                                    if distance2 < suggestion_split_best.distance {
                                        suggestion_split_best = Suggestion::empty();
                                    }
                                }
                                let count2: i32 = match self.bigrams.get(&*suggestion_split.term) {
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
                                            if suggestion_split.term
                                                == input.get_word(&term_list1[i])
                                            {
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
                                        } else if suggestion_split.term
                                            == input.get_word(&term_list1[i])
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
                                                as i32,
                                        )
                                    }
                                };
                                suggestion_split.distance = distance2;
                                suggestion_split.count = count2;

                                //early termination of split
                                if suggestion_split_best.term == ""
                                    || suggestion_split.count > suggestion_split_best.count
                                {
                                    suggestion_split_best = suggestion_split.clone();
                                }
                            }
                        }
                    }

                    if suggestion_split_best.term != "" {
                        //select best suggestion for split pair
                        suggestion_parts.push(suggestion_split_best.clone());
                    } else {
                        let mut si = Suggestion::empty();
                        // NOTE: this effectively clamps si_count to a certain minimum value, which it can't go below
                        let si_count: f64 = 10f64
                            / ((10i32).saturating_pow(
                                self.string_strategy.len(input.get_word(&term_list1[i])) as u32,
                            )) as f64;

                        si.term = input.get_word(&term_list1[i]).to_string();
                        si.count = si_count as i32;
                        si.distance = edit_distance_max + 1;
                        suggestion_parts.push(si);
                    }
                } else {
                    let mut si = Suggestion::empty();
                    // NOTE: this effectively clamps si_count to a certain minimum value, which it can't go below
                    let si_count: f64 = 10f64
                        / ((10i32).saturating_pow(
                            self.string_strategy.len(input.get_word(&term_list1[i])) as u32,
                        )) as f64;

                    si.term = input.get_word(&term_list1[i]).to_owned();
                    si.count = si_count as i32;
                    si.distance = edit_distance_max + 1;
                    suggestion_parts.push(si);
                }
            }
        }

        for si in suggestion_parts {
            // TODO: key
            let mut l = g.push_word(&si.term, None);
            l.set_suffix(g.push_str(" "));
            p.push_lexeme(l);
        }
    }

    /// Divides a string into words by inserting missing spaces at the appropriate positions
    ///
    ///
    /// # Arguments
    ///
    /// * `input` - The word being segmented.
    /// * `max_edit_distance` - The maximum edit distance between input and suggested words.
    ///
    /// # Examples
    ///
    /// ```
    /// use spellcheck::symspell::{SymSpell, Verbosity};
    ///
    /// let mut symspell: SymSpell = SymSpell::default();
    /// symspell.load_dictionary("it 2\nwas 2", 0, 1, " ");
    /// symspell.word_segmentation("itwas", 2);
    /// ```
    pub fn word_segmentation(&self, input: &str, max_edit_distance: i32) -> Composition {
        let input = self.string_strategy.prepare(input);
        let asize = self.string_strategy.len(&input);

        let mut ci: usize = 0;
        let mut compositions: Vec<Composition> = vec![Composition::empty(); asize];

        for j in 0..asize {
            let imax = cmp::min(asize - j, self.max_length as usize);
            for i in 1..=imax {
                let top_prob_log: f64;

                let mut part = self.string_strategy.slice(&input, j, j + i);

                let mut sep_len = 0;
                let mut top_ed: i32 = 0;

                let first_char = self.string_strategy.at(&part, 0).unwrap();
                if first_char.is_whitespace() {
                    part = self.string_strategy.remove(&part, 0);
                } else {
                    sep_len = 1;
                }

                top_ed += part.len() as i32;

                part = part.replace(" ", "");

                top_ed -= part.len() as i32;

                let results = self.lookup(&part, Verbosity::Top, max_edit_distance);

                if !results.is_empty() && results[0].distance == 0 {
                    top_prob_log =
                        (results[0].count as f64 / self.corpus_word_count as f64).log10();
                } else {
                    top_ed += part.len() as i32;
                    top_prob_log = (10.0
                        / (self.corpus_word_count as f64 * 10.0f64.powf(part.len() as f64)))
                    .log10();
                }

                let di = (i + ci) % asize;
                // set values in first loop
                if j == 0 {
                    compositions[i - 1] = Composition {
                        segmented_string: part.to_owned(),
                        distance_sum: top_ed,
                        prob_log_sum: top_prob_log,
                    };
                } else if i as i32 == self.max_length
                    || (((compositions[ci].distance_sum + top_ed == compositions[di].distance_sum)
                        || (compositions[ci].distance_sum + sep_len + top_ed
                            == compositions[di].distance_sum))
                        && (compositions[di].prob_log_sum
                            < compositions[ci].prob_log_sum + top_prob_log))
                    || (compositions[ci].distance_sum + sep_len + top_ed
                        < compositions[di].distance_sum)
                {
                    compositions[di] = Composition {
                        segmented_string: format!("{} {}", compositions[ci].segmented_string, part),
                        distance_sum: compositions[ci].distance_sum + sep_len + top_ed,
                        prob_log_sum: compositions[ci].prob_log_sum + top_prob_log,
                    };
                }
            }
            if j != 0 {
                ci += 1;
            }
            ci = if ci == asize { 0 } else { ci };
        }
        compositions[ci].to_owned()
    }

    fn delete_in_suggestion_prefix(
        &self,
        delete: &str,
        delete_len: i32,
        suggestion: &str,
        suggestion_len: i32,
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

    fn create_dictionary_entry<K>(&mut self, key: K, count: i32) -> bool
    where
        K: Clone + AsRef<str> + Into<String>,
    {
        if count < self.count_threshold {
            return false;
        }

        let key_clone = key.clone().into().into_boxed_str();

        match self.wordfreq.get(key.as_ref()) {
            Some(i) => {
                let updated_count = if i32::MAX - i > count {
                    i + count
                } else {
                    i32::MAX
                };
                self.wordfreq.insert(key_clone, updated_count);
                return false;
            }
            None => {
                let id = self.idword.len() as u32 + 1;
                self.wordid.insert(key.clone().into().into_boxed_str(), id);
                self.idword.insert(id, key.clone().into().into_boxed_str());
                self.wordfreq.insert(key_clone, count);
            }
        }

        let key_len = self.string_strategy.len(key.as_ref());

        if key_len as i32 > self.max_length {
            self.max_length = key_len as i32;
        }

        let edits = self.edits_prefix(key.as_ref());
        let key_id = self.wordid.get(key.as_ref()).unwrap();

        for delete in edits {
            let delete_hash = self.get_string_hash(&delete);

            self.deletes
                .entry(delete_hash)
                .and_modify(|e| e.push(key_id.clone()))
                .or_insert_with(|| vec![key_id.clone()]);
        }

        true
    }

    fn edits_prefix(&self, key: &str) -> HashSet<String> {
        let mut hash_set = HashSet::new();

        let key_len = self.string_strategy.len(key) as i32;

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

    fn edits(&self, word: &str, edit_distance: i32, delete_words: &mut HashSet<String>) {
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
        max_edit_distance: i32,
        input: &str,
        input_len: i32,
        candidate_len: i32,
        suggestion: &str,
        suggestion_len: i32,
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
        xxh64::xxh64(s.as_bytes(), 42)
    }
}
