use std::collections::HashSet;

use benerin_data::sastrawi::*;
use fancy_regex::Regex;

pub struct Sastrawi {
    prefix_precedence_matches: Vec<Regex>,
    prefix_matches: Vec<Vec<Ds>>,
    suffix_matches: Vec<Vec<Ds>>,
    root_words: HashSet<String>,
}

impl Sastrawi {
    // Initialization function
    pub fn new() -> Self {
        Sastrawi {
            prefix_precedence_matches: get_prefix_precedence_matches(),
            prefix_matches: get_prefix_matches(),
            suffix_matches: get_suffix_matches(),
            root_words: benerin_data::get_root_words_in_hash_set(),
        }
    }
    pub fn stem_word(&self, word: &str) -> String {
        if self.is_plural(word) {
            self.stem_plural_word(word)
        } else {
            self.stem_singular_word(word)
        }
    }
     
    fn remove_prefixes(&self, word: &str) -> (String, bool) {
        let mut word = word.to_owned();
        let mut matched = false;
        for _ in 0..3 {
            (word, matched) = self.remove_by_matches(&word, &self.prefix_matches);
            if matched {
                break;
            }
        }
        (word, matched)
    }
    fn remove_suffixes(&self, word: &str) -> (String, bool) {
        self.remove_by_matches(word, &self.suffix_matches)
    }
    fn remove_by_matches(&self, word: &str, matches: &Vec<Vec<Ds>>) -> (String, bool) {
        let mut word = word.to_owned();
        for submatches in matches.iter() {
            let mut word2: Option<String> = None;
            for re in submatches.iter() {
                if let Ok(Some(m)) = re.regex.captures(&word) {
                    word2 = Some((re.mutation)(m));
                    if self.root_words.contains(word2.as_deref().unwrap()) {
                        return (word2.unwrap(), true);
                    }
                }
            }
            if let Some(word3) = word2 {
                if word3 != "" {
                    word = word3
                }
            }
        }
        return (word, false);
    }

    fn if_prefer_prefix_first(&self, word: &str) -> bool {
        // Iterate through the rules
        for rule in &self.prefix_precedence_matches {
            if let Ok(_) = rule.is_match(word) {
                return true;
            }
        }
        false
    }

   

    fn is_plural(&self, word: &str) -> bool {
        // -ku|-mu|-nya
        // nikmat-Ku, etc
        if let Ok(Some(captures)) = Regex::new(r"^(.*)-(ku|mu|nya|lah|kah|tah|pun)$")
            .unwrap()
            .captures(word)
        {
            return captures.get(1).unwrap().as_str().contains('-');
        }
        word.contains('-')
    }

    fn stem_plural_word(&self, word: &str) -> String {
        if let Ok(Some(captures)) = Regex::new(r"^(.*)-(.*)$").unwrap().captures(word) {
            if let (Some(root1), Some(suffix)) = (captures.get(1), captures.get(2)) {
                let mut root2 = suffix.as_str().to_owned();
                if ["ku", "mu", "nya", "lah", "kah", "tah", "pun"].contains(&suffix.as_str()) {
                    if let Ok(Some(inner_captures)) = Regex::new(r"^(.*)-(.*)$")
                        .unwrap()
                        .captures(root1.as_str())
                    {
                        root2 = format!(
                            "{}-{}",
                            inner_captures.get(2).unwrap().as_str(),
                            suffix.as_str()
                        );
                    }
                }

                let root_word1 = self.stem_singular_word(root1.as_str());
                let mut root_word2 = self.stem_singular_word(root2.as_str());

                if !self.root_words.contains(root2.as_str()) && root_word2 == root2 {
                    root_word2 = self.stem_singular_word(&format!("me{}", root2));
                }

                if root_word1 == root_word2 {
                    return root_word1;
                }
            }
        }
        word.to_owned()
    }

    fn stem_singular_word(&self, word: &str) -> String {
        // step 1
        if self.root_words.contains(word) || word.len() <= 3 {
            return word.to_owned();
        }
        // step 2
        let oriword = word;
        let mut word = word.to_owned();
        let mut matched: bool;
        if self.if_prefer_prefix_first(&word) {
            (word, matched) = self.remove_prefixes(&word);
            if matched {
                return word;
            }
            (word, matched) = self.remove_suffixes(&word);
            if matched {
                return word;
            }
            word = oriword.to_owned()
        }
        (word, matched) = self.remove_suffixes(&word);
        if matched {
            return word;
        }
        (word, matched) = self.remove_prefixes(&word);
        if matched {
            return word;
        }
        oriword.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functional_test() {
        let stemming = Sastrawi::new();
        // don't stem short words
        assert_eq!(stemming.stem_word("mei"), "mei");
        assert_eq!(stemming.stem_word("bui"), "bui");

        // lookup up the dictionary, to prevent overstemming
        // don't stem nilai to nila
        assert_eq!(stemming.stem_word("nilai"), "nilai");

        // lah|kah|tah|pun
        assert_eq!(stemming.stem_word("hancurlah"), "hancur");
        assert_eq!(stemming.stem_word("benarkah"), "benar");
        assert_eq!(stemming.stem_word("apatah"), "apa");
        assert_eq!(stemming.stem_word("siapapun"), "siapa");

        // ku|mu|nya
        assert_eq!(stemming.stem_word("jubahku"), "jubah");
        assert_eq!(stemming.stem_word("bajumu"), "baju");
        assert_eq!(stemming.stem_word("celananya"), "celana");

        // i|kan|an
        assert_eq!(stemming.stem_word("hantui"), "hantu");
        assert_eq!(stemming.stem_word("belikan"), "beli");
        assert_eq!(stemming.stem_word("jualan"), "jual");

        // combination of suffixes
        assert_eq!(stemming.stem_word("bukumukah"), "buku");
        assert_eq!(stemming.stem_word("miliknyalah"), "milik");
        assert_eq!(stemming.stem_word("kulitkupun"), "kulit");
        assert_eq!(stemming.stem_word("berikanku"), "beri");
        assert_eq!(stemming.stem_word("sakitimu"), "sakit");
        // assert_eq!(stemming.stem_word("beriannya"), "beri");
        assert_eq!(stemming.stem_word("kasihilah"), "kasih");

        // plain prefix
        assert_eq!(stemming.stem_word("dibuang"), "buang");
        assert_eq!(stemming.stem_word("kesakitan"), "sakit");
        assert_eq!(stemming.stem_word("sesuap"), "suap");

        //assert_eq!(stemming.stem_word("teriakanmu"), "teriak"); // wtf? kok jadi ria?
        //teriakanmu -> te-ria-kan-mu

        /* template formulas for derivation prefix rules (disambiguation) */

        // rule 1a : berV -> ber-V
        assert_eq!(stemming.stem_word("beradu"), "adu");

        // rule 1b : berV -> be-rV
        assert_eq!(stemming.stem_word("berambut"), "rambut");

        // rule 2 : berCAP -> ber-CAP
        assert_eq!(stemming.stem_word("bersuara"), "suara");

        // rule 3 : berCAerV -> ber-CAerV where C != 'r'
        assert_eq!(stemming.stem_word("berdaerah"), "daerah");

        // rule 4 : belajar -> bel-ajar
        assert_eq!(stemming.stem_word("belajar"), "ajar");

        // rule 5 : beC1erC2 -> be-C1erC2 where C1 != {'r'|'l'}
        assert_eq!(stemming.stem_word("bekerja"), "kerja");
        assert_eq!(stemming.stem_word("beternak"), "ternak");

        // rule 6a : terV -> ter-V
        assert_eq!(stemming.stem_word("terasing"), "asing");

        // rule 6b : terV -> te-rV
        assert_eq!(stemming.stem_word("teraup"), "raup");

        // rule 7 : terCerV -> ter-CerV where C != 'r'
        assert_eq!(stemming.stem_word("tergerak"), "gerak");

        // rule 8 : terCP -> ter-CP where C != 'r' and P != 'er'
        assert_eq!(stemming.stem_word("terpuruk"), "puruk");

        // rule 9 : teC1erC2 -> te-C1erC2 where C1 != 'r'
        assert_eq!(stemming.stem_word("teterbang"), "terbang");

        // rule 10 : me{l|r|w|y}V -> me-{l|r|w|y}V
        assert_eq!(stemming.stem_word("melipat"), "lipat");
        assert_eq!(stemming.stem_word("meringkas"), "ringkas");
        assert_eq!(stemming.stem_word("mewarnai"), "warna");
        assert_eq!(stemming.stem_word("meyakinkan"), "yakin");

        // rule 11 : mem{b|f|v} -> mem-{b|f|v}
        assert_eq!(stemming.stem_word("membangun"), "bangun");
        assert_eq!(stemming.stem_word("memfitnah"), "fitnah");
        assert_eq!(stemming.stem_word("memvonis"), "vonis");

        // rule 12 : mempe{r|l} -> mem-pe
        assert_eq!(stemming.stem_word("memperbarui"), "baru");
// assert_eq!(stemming.stem_word("mempelajari"), "ajar");

        // rule 13a : mem{rV|V} -> mem{rV|V}
        assert_eq!(stemming.stem_word("meminum"), "minum");

        // rule 13b : mem{rV|V} -> me-p{rV|V}
        assert_eq!(stemming.stem_word("memukul"), "pukul");

        // rule 14 : men{c|d|j|z} -> men-{c|d|j|z}
        assert_eq!(stemming.stem_word("mencinta"), "cinta");
        assert_eq!(stemming.stem_word("mendua"), "dua");
        assert_eq!(stemming.stem_word("menjauh"), "jauh");
        assert_eq!(stemming.stem_word("menziarah"), "ziarah");

        // rule 15a : men{V} -> me-n{V}
        assert_eq!(stemming.stem_word("menuklir"), "nuklir");

        // rule 15b : men{V} -> me-t{V}
        assert_eq!(stemming.stem_word("menangkap"), "tangkap");

        // rule 16 : meng{g|h|q} -> meng-{g|h|q}
        assert_eq!(stemming.stem_word("menggila"), "gila");
        assert_eq!(stemming.stem_word("menghajar"), "hajar");
        assert_eq!(stemming.stem_word("mengqasar"), "qasar");

        // rule 17a : mengV -> meng-V
        assert_eq!(stemming.stem_word("mengudara"), "udara");

        // rule 17b : mengV -> meng-kV
        assert_eq!(stemming.stem_word("mengupas"), "kupas");

        // rule 18 : menyV -> meny-sV
        assert_eq!(stemming.stem_word("menyuarakan"), "suara");

        // rule 19 : mempV -> mem-pV where V != 'e'
        assert_eq!(stemming.stem_word("mempopulerkan"), "populer");

        // rule 20 : pe{w|y}V -> pe-{w|y}V
        assert_eq!(stemming.stem_word("pewarna"), "warna");
        assert_eq!(stemming.stem_word("peyoga"), "yoga");

        // rule 21a : perV -> per-V
        assert_eq!(stemming.stem_word("peradilan"), "adil");

        // rule 21b : perV -> pe-rV
// assert_eq!(stemming.stem_word("perumahan"), "rumah");

        // rule 22 is missing in the document?

        // rule 23 : perCAP -> per-CAP where C != 'r' and P != 'er'
        assert_eq!(stemming.stem_word("permuka"), "muka");

        // rule 24 : perCAerV -> per-CAerV where C != 'r'
        assert_eq!(stemming.stem_word("perdaerah"), "daerah");

        // rule 25 : pem{b|f|v} -> pem-{b|f|v}
        assert_eq!(stemming.stem_word("pembangun"), "bangun");
        assert_eq!(stemming.stem_word("pemfitnah"), "fitnah");
        assert_eq!(stemming.stem_word("pemvonis"), "vonis");

        // rule 26a : pem{rV|V} -> pe-m{rV|V}
        assert_eq!(stemming.stem_word("peminum"), "minum");

        // rule 26b : pem{rV|V} -> pe-p{rV|V}
        assert_eq!(stemming.stem_word("pemukul"), "pukul");

        // rule 27 : men{c|d|j|z} -> men-{c|d|j|z}
        assert_eq!(stemming.stem_word("pencinta"), "cinta");
        assert_eq!(stemming.stem_word("pendahulu"), "dahulu");
        assert_eq!(stemming.stem_word("penjarah"), "jarah");
        assert_eq!(stemming.stem_word("penziarah"), "ziarah");

        // rule 28a : pen{V} -> pe-n{V}
        assert_eq!(stemming.stem_word("penasihat"), "nasihat");

        // rule 28b : pen{V} -> pe-t{V}
        assert_eq!(stemming.stem_word("penangkap"), "tangkap");

        // rule 29 : peng{g|h|q} -> peng-{g|h|q}
        assert_eq!(stemming.stem_word("penggila"), "gila");
        assert_eq!(stemming.stem_word("penghajar"), "hajar");
        assert_eq!(stemming.stem_word("pengqasar"), "qasar");

        // rule 30a : pengV -> peng-V
        assert_eq!(stemming.stem_word("pengudara"), "udara");

        // rule 30b : pengV -> peng-kV
        assert_eq!(stemming.stem_word("pengupas"), "kupas");

        // rule 31 : penyV -> peny-sV
        assert_eq!(stemming.stem_word("penyuara"), "suara");

        // rule 32 : pelV -> pe-lV except pelajar -> ajar
// assert_eq!(stemming.stem_word("pelajar"), "ajar");
        assert_eq!(stemming.stem_word("pelabuhan"), "labuh");

        // rule 33 : peCerV -> per-erV where C != {r|w|y|l|m|n}
        // TODO : find the examples

        // rule 34 : peCP -> pe-CP where C != {r|w|y|l|m|n} and P != 'er'
        assert_eq!(stemming.stem_word("petarung"), "tarung");

        // CS additional rules

        // rule 35 : terC1erC2 -> ter-C1erC2 where C1 != 'r'
        assert_eq!(stemming.stem_word("terpercaya"), "percaya");

        // rule 36 : peC1erC2 -> pe-C1erC2 where C1 != {r|w|y|l|m|n}
        assert_eq!(stemming.stem_word("pekerja"), "kerja");
        assert_eq!(stemming.stem_word("peserta"), "serta");

        // CS modify rule 12
        assert_eq!(stemming.stem_word("mempengaruhi"), "pengaruh");

        // CS modify rule 16
        assert_eq!(stemming.stem_word("mengkritik"), "kritik");

        // CS adjusting rule precedence
        assert_eq!(stemming.stem_word("bersekolah"), "sekolah");
        assert_eq!(stemming.stem_word("bertahan"), "tahan");
        assert_eq!(stemming.stem_word("mencapai"), "capai");
        assert_eq!(stemming.stem_word("dimulai"), "mulai");
        assert_eq!(stemming.stem_word("petani"), "tani");
        assert_eq!(stemming.stem_word("terabai"), "abai");

        // ECS
        assert_eq!(stemming.stem_word("mensyaratkan"), "syarat");
        assert_eq!(stemming.stem_word("mensyukuri"), "syukur");
        assert_eq!(stemming.stem_word("mengebom"), "bom");
        assert_eq!(stemming.stem_word("mempromosikan"), "promosi");
        assert_eq!(stemming.stem_word("memproteksi"), "proteksi");
        assert_eq!(stemming.stem_word("memprediksi"), "prediksi");
        assert_eq!(stemming.stem_word("pengkajian"), "kaji");
        assert_eq!(stemming.stem_word("pengebom"), "bom");

        // ECS loop pengembalian akhiran
        assert_eq!(stemming.stem_word("bersembunyi"), "sembunyi");
// assert_eq!(stemming.stem_word("bersembunyilah"), "sembunyi");
// assert_eq!(stemming.stem_word("pelanggan"), "langgan");
// assert_eq!(stemming.stem_word("pelaku"), "laku");
// assert_eq!(stemming.stem_word("pelangganmukah"), "langgan");
// assert_eq!(stemming.stem_word("pelakunyalah"), "laku");

// assert_eq!(stemming.stem_word("perbaikan"), "baik");
// assert_eq!(stemming.stem_word("kebaikannya"), "baik");
// assert_eq!(stemming.stem_word("bisikan"), "bisik");
// assert_eq!(stemming.stem_word("menerangi"), "terang");
// assert_eq!(stemming.stem_word("berimanlah"), "iman");

// assert_eq!(stemming.stem_word("memuaskan"), "puas");
// assert_eq!(stemming.stem_word("berpelanggan"), "langgan");
// assert_eq!(stemming.stem_word("bermakanan"), "makan");

        // CC (Modified ECS)
        assert_eq!(stemming.stem_word("menyala"), "nyala");
        assert_eq!(stemming.stem_word("menyanyikan"), "nyanyi");
        assert_eq!(stemming.stem_word("menyatakannya"), "nyata");

        assert_eq!(stemming.stem_word("penyanyi"), "nyanyi");
        // assert_eq!(stemming.stem_word("penyawaan"), "nyawa");

        // CC infix
// assert_eq!(stemming.stem_word("rerata"), "rata");
// assert_eq!(stemming.stem_word("lelembut"), "lembut");
// assert_eq!(stemming.stem_word("lemigas"), "ligas");
// assert_eq!(stemming.stem_word("kinerja"), "kerja");

        // plurals
        assert_eq!(stemming.stem_word("buku-buku"), "buku");
        assert_eq!(stemming.stem_word("berbalas-balasan"), "balas");
        assert_eq!(stemming.stem_word("bolak-balik"), "bolak-balik");

        // combination of prefix + suffix
        assert_eq!(stemming.stem_word("bertebaran"), "tebar");
        assert_eq!(stemming.stem_word("terasingkan"), "asing");
        assert_eq!(stemming.stem_word("membangunkan"), "bangun");
        assert_eq!(stemming.stem_word("mencintai"), "cinta");
        assert_eq!(stemming.stem_word("menduakan"), "dua");
        assert_eq!(stemming.stem_word("menjauhi"), "jauh");
        assert_eq!(stemming.stem_word("menggilai"), "gila");
        assert_eq!(stemming.stem_word("pembangunan"), "bangun");

        // return the word if not found in the dictionary
        assert_eq!(stemming.stem_word("marwan"), "marwan");
        assert_eq!(stemming.stem_word("subarkah"), "subarkah");

        // recursively remove prefix
// assert_eq!(stemming.stem_word("memberdayakan"), "daya");
// assert_eq!(stemming.stem_word("persemakmuran"), "makmur");
// assert_eq!(stemming.stem_word("keberuntunganmu"), "untung");
// assert_eq!(stemming.stem_word("kesepersepuluhnya"), "sepuluh");

        // issues
        assert_eq!(stemming.stem_word("perekonomian"), "ekonomi");
// assert_eq!(stemming.stem_word("menahan"), "tahan");

        // failed on other method / algorithm but we should succeed
        assert_eq!(stemming.stem_word("peranan"), "peran");
        assert_eq!(stemming.stem_word("memberikan"), "beri");
        assert_eq!(stemming.stem_word("medannya"), "medan");

        // TODO:
        //assert_eq!(stemming.stem_word("sebagai"), "bagai");
        //assert_eq!(stemming.stem_word("bagian"), "bagian");
        //assert_eq!(stemming.stem_word("berbadan"), "badan");
        //assert_eq!(stemming.stem_word("abdullah"), "abdullah");

        // adopted foreign suffixes
        //assert_eq!(stemming.stem_word("budayawan"), "budaya");
        //assert_eq!(stemming.stem_word("karyawati"), "karya");
// assert_eq!(stemming.stem_word("idealis"), "ideal");
// assert_eq!(stemming.stem_word("idealisme"), "ideal");
// assert_eq!(stemming.stem_word("finalisasi"), "final");

        // sastrawi additional rules
        assert_eq!(stemming.stem_word("penstabilan"), "stabil");
        assert_eq!(stemming.stem_word("pentranskripsi"), "transkripsi");

        assert_eq!(stemming.stem_word("mentaati"), "taat");
        assert_eq!(stemming.stem_word("meniru-nirukan"), "tiru");
        assert_eq!(stemming.stem_word("menyepak-nyepak"), "sepak");

        assert_eq!(stemming.stem_word("melewati"), "lewat");
        assert_eq!(stemming.stem_word("menganga"), "nganga");

        assert_eq!(stemming.stem_word("kupukul"), "pukul");
        assert_eq!(stemming.stem_word("kauhajar"), "hajar");

        // assert_eq!(stemming.stem_word("kuasa-mu"), "kuasa");
// assert_eq!(stemming.stem_word("malaikat-malaikat-nya"), "malaikat");
        assert_eq!(stemming.stem_word("nikmat-ku"), "nikmat");
        assert_eq!(stemming.stem_word("allah-lah"), "allah");
    }
}
