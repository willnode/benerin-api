// POSTEMI: POhon (tree based) STEMming untuk bahasa Indonesia
// 100% test case pass on Sastrawi with less than 100 LoC
// Original algorithm (c) Wildan Mubarok

use std::collections::{HashMap, HashSet};

use fancy_regex::Regex;

pub struct Postemi {
    prefix_matches: HashMap<String, (bool, Vec<String>)>,
    suffix_matches: HashMap<String, bool>,
    root_words: HashSet<String>,
    plural_detect: Regex,
}

impl Postemi {
    // Initialization function
    pub fn new() -> Self {
        Postemi {
            prefix_matches: benerin_data::get_prefiks_indexed_in_hash_map(),
            suffix_matches: benerin_data::get_suffiks_indexed_in_hash_map(),
            root_words: benerin_data::get_root_words_in_hash_set(),
            plural_detect: Regex::new(r"(\w+)-\1").unwrap(),
        }
    }

    // Initialization function
    pub fn stem_word(&self, word: &str) -> Option<&str> {
        let mut suffix_offsets = vec![0];
        let mut prefix_offsets: Vec<(usize, Vec<String>)> = vec![(0, vec![])];
        let mut word = word;
        let single_w: String;
        if let Some(_) = word.find('-') {
            single_w = self.plural_detect.replace(&word, "$1").into_owned();
            word = &single_w;
        }
        let mut s = 0;
        loop {
            s += 1;
            if word.len() <= s {
                break;
            }
            match self.suffix_matches.get(&word[word.len() - s..word.len()]) {
                Some(true) => suffix_offsets.push(s),
                Some(false) => continue,
                None => break,
            }
        }
        let mut p = 0;
        loop {
            p += 1;
            if word.len() <= p {
                break;
            }
            match self.prefix_matches.get(&word[0..p]) {
                Some((true, v)) => prefix_offsets.push((p, v.to_vec())),
                Some((false, _)) => continue,
                None => break,
            }
        }
        let mut candidates: Vec<(usize, &str)> = vec![];
        for (p, pf) in prefix_offsets.iter() {
            for s in suffix_offsets.iter() {
                let m = &word[*p..word.len() - *s];
                if let Some(mm) = self.root_words.get(m) {
                    candidates.push((mm.len(), mm));
                }
                for pf in pf.iter() {
                    let m = pf.to_owned() + &word[*p..word.len() - *s];
                    if let Some(mm) = self.root_words.get(&m) {
                        candidates.push((mm.len(), mm));
                    }
                }
            }
        }
        if candidates.len() > 0 {
            let mut highest_candidate: &(usize, &str) = &candidates[0];
            for c in candidates.iter() {
                if highest_candidate.0 < c.0 {
                    highest_candidate = c
                }
            }
            return Some(highest_candidate.1);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functional_test() {
        let stemming = Postemi::new();
        assert_eq!(stemming.stem_word("rerata"), Some("rata"));
        // don't stem short words
        assert_eq!(stemming.stem_word("mei"), Some("mei"));
        assert_eq!(stemming.stem_word("bui"), Some("bui"));

        // lookup up the dictionary, to prevent overstemming
        // don't stem nilai to nila
        assert_eq!(stemming.stem_word("nilai"), Some("nilai"));

        // lah|kah|tah|pun
        assert_eq!(stemming.stem_word("hancurlah"), Some("hancur"));
        assert_eq!(stemming.stem_word("benarkah"), Some("benar"));
        assert_eq!(stemming.stem_word("apatah"), Some("apa"));
        assert_eq!(stemming.stem_word("siapapun"), Some("siapa"));

        // ku|mu|nya
        assert_eq!(stemming.stem_word("jubahku"), Some("jubah"));
        assert_eq!(stemming.stem_word("bajumu"), Some("baju"));
        assert_eq!(stemming.stem_word("celananya"), Some("celana"));

        // i|kan|an
        assert_eq!(stemming.stem_word("hantui"), Some("hantu"));
        assert_eq!(stemming.stem_word("belikan"), Some("beli"));
        assert_eq!(stemming.stem_word("jualan"), Some("jual"));

        // combination of suffixes
        assert_eq!(stemming.stem_word("miliknyalah"), Some("milik"));
        assert_eq!(stemming.stem_word("bukumukah"), Some("buku"));
        assert_eq!(stemming.stem_word("kulitkupun"), Some("kulit"));
        assert_eq!(stemming.stem_word("berikanku"), Some("beri"));
        assert_eq!(stemming.stem_word("sakitimu"), Some("sakit"));
        assert_eq!(stemming.stem_word("beriannya"), Some("beri"));
        assert_eq!(stemming.stem_word("kasihilah"), Some("kasih"));

        // plain prefix
        assert_eq!(stemming.stem_word("dibuang"), Some("buang"));
        assert_eq!(stemming.stem_word("kesakitan"), Some("sakit"));
        assert_eq!(stemming.stem_word("sesuap"), Some("suap"));

        assert_eq!(stemming.stem_word("teriakanmu"), Some("teriak"));

        /* template formulas for derivation prefix rules (disambiguation) */

        // rule 1a : berV -> ber-V
        assert_eq!(stemming.stem_word("beradu"), Some("adu"));

        // rule 1b : berV -> be-rV
        assert_eq!(stemming.stem_word("berambut"), Some("rambut"));

        // rule 2 : berCAP -> ber-CAP
        assert_eq!(stemming.stem_word("bersuara"), Some("suara"));

        // rule 3 : berCAerV -> ber-CAerV where C != 'r'
        assert_eq!(stemming.stem_word("berdaerah"), Some("daerah"));

        // rule 4 : belajar -> bel-ajar
        assert_eq!(stemming.stem_word("belajar"), Some("ajar"));

        // rule 5 : beC1erC2 -> be-C1erC2 where C1 != {'r'|'l'}
        assert_eq!(stemming.stem_word("bekerja"), Some("kerja"));
        assert_eq!(stemming.stem_word("beternak"), Some("ternak"));

        // rule 6a : terV -> ter-V
        assert_eq!(stemming.stem_word("terasing"), Some("asing"));

        // rule 6b : terV -> te-rV
        assert_eq!(stemming.stem_word("teraup"), Some("raup"));

        // rule 7 : terCerV -> ter-CerV where C != 'r'
        assert_eq!(stemming.stem_word("tergerak"), Some("gerak"));

        // rule 8 : terCP -> ter-CP where C != 'r' and P != 'er'
        assert_eq!(stemming.stem_word("terpuruk"), Some("puruk"));

        // rule 9 : teC1erC2 -> te-C1erC2 where C1 != 'r'
        assert_eq!(stemming.stem_word("teterbang"), Some("terbang"));

        // rule 10 : me{l|r|w|y}V -> me-{l|r|w|y}V
        assert_eq!(stemming.stem_word("melipat"), Some("lipat"));
        assert_eq!(stemming.stem_word("meringkas"), Some("ringkas"));
        assert_eq!(stemming.stem_word("mewarnai"), Some("warna"));
        assert_eq!(stemming.stem_word("meyakinkan"), Some("yakin"));

        // rule 11 : mem{b|f|v} -> mem-{b|f|v}
        assert_eq!(stemming.stem_word("membangun"), Some("bangun"));
        assert_eq!(stemming.stem_word("memfitnah"), Some("fitnah"));
        assert_eq!(stemming.stem_word("memvonis"), Some("vonis"));

        // rule 12 : mempe{r|l} -> mem-pe
        assert_eq!(stemming.stem_word("memperbarui"), Some("baru"));
        assert_eq!(stemming.stem_word("mempelajari"), Some("ajar"));

        // rule 13a : mem{rV|V} -> mem{rV|V}
        assert_eq!(stemming.stem_word("meminum"), Some("minum"));

        // rule 13b : mem{rV|V} -> me-p{rV|V}
        assert_eq!(stemming.stem_word("memukul"), Some("pukul"));

        // rule 14 : men{c|d|j|z} -> men-{c|d|j|z}
        assert_eq!(stemming.stem_word("mencinta"), Some("cinta"));
        assert_eq!(stemming.stem_word("mendua"), Some("dua"));
        assert_eq!(stemming.stem_word("menjauh"), Some("jauh"));
        assert_eq!(stemming.stem_word("menziarah"), Some("ziarah"));

        // rule 15a : men{V} -> me-n{V}
        assert_eq!(stemming.stem_word("menuklir"), Some("nuklir"));

        // rule 15b : men{V} -> me-t{V}
        assert_eq!(stemming.stem_word("menangkap"), Some("tangkap"));

        // rule 16 : meng{g|h|q} -> meng-{g|h|q}
        assert_eq!(stemming.stem_word("menggila"), Some("gila"));
        assert_eq!(stemming.stem_word("menghajar"), Some("hajar"));
        assert_eq!(stemming.stem_word("mengqasar"), Some("qasar"));

        // rule 17a : mengV -> meng-V
        assert_eq!(stemming.stem_word("mengudara"), Some("udara"));

        // rule 17b : mengV -> meng-kV
        assert_eq!(stemming.stem_word("mengupas"), Some("kupas"));

        // rule 18 : menyV -> meny-sV
        assert_eq!(stemming.stem_word("menyuarakan"), Some("suara"));

        // rule 19 : mempV -> mem-pV where V != 'e'
        assert_eq!(stemming.stem_word("mempopulerkan"), Some("populer"));

        // rule 20 : pe{w|y}V -> pe-{w|y}V
        assert_eq!(stemming.stem_word("pewarna"), Some("warna"));
        assert_eq!(stemming.stem_word("peyoga"), Some("yoga"));

        // rule 21a : perV -> per-V
        assert_eq!(stemming.stem_word("peradilan"), Some("adil"));

        // rule 21b : perV -> pe-rV
        assert_eq!(stemming.stem_word("perumahan"), Some("rumah"));

        // rule 22 is missing in the document?

        // rule 23 : perCAP -> per-CAP where C != 'r' and P != 'er'
        assert_eq!(stemming.stem_word("permuka"), Some("muka"));

        // rule 24 : perCAerV -> per-CAerV where C != 'r'
        assert_eq!(stemming.stem_word("perdaerah"), Some("daerah"));

        // rule 25 : pem{b|f|v} -> pem-{b|f|v}
        assert_eq!(stemming.stem_word("pembangun"), Some("bangun"));
        assert_eq!(stemming.stem_word("pemfitnah"), Some("fitnah"));
        assert_eq!(stemming.stem_word("pemvonis"), Some("vonis"));
        assert_eq!(stemming.stem_word("pemrograman"), Some("program"));

        // rule 26a : pem{rV|V} -> pe-m{rV|V}
        assert_eq!(stemming.stem_word("peminum"), Some("minum"));

        // rule 26b : pem{rV|V} -> pe-p{rV|V}
        assert_eq!(stemming.stem_word("pemukul"), Some("pukul"));

        // rule 27 : men{c|d|j|z} -> men-{c|d|j|z}
        assert_eq!(stemming.stem_word("pencinta"), Some("cinta"));
        assert_eq!(stemming.stem_word("pendahulu"), Some("dahulu"));
        assert_eq!(stemming.stem_word("penjarah"), Some("jarah"));
        assert_eq!(stemming.stem_word("penziarah"), Some("ziarah"));

        // rule 28a : pen{V} -> pe-n{V}
        assert_eq!(stemming.stem_word("penasihat"), Some("nasihat"));

        // rule 28b : pen{V} -> pe-t{V}
        assert_eq!(stemming.stem_word("penangkap"), Some("tangkap"));

        // rule 29 : peng{g|h|q} -> peng-{g|h|q}
        assert_eq!(stemming.stem_word("penggila"), Some("gila"));
        assert_eq!(stemming.stem_word("penghajar"), Some("hajar"));
        assert_eq!(stemming.stem_word("pengqasar"), Some("qasar"));

        // rule 30a : pengV -> peng-V
        assert_eq!(stemming.stem_word("pengudara"), Some("udara"));

        // rule 30b : pengV -> peng-kV
        assert_eq!(stemming.stem_word("pengupas"), Some("kupas"));

        // rule 31 : penyV -> peny-sV
        assert_eq!(stemming.stem_word("penyuara"), Some("suara"));

        // rule 32 : pelV -> pe-lV except pelajar -> ajar
        assert_eq!(stemming.stem_word("pelajar"), Some("ajar"));
        assert_eq!(stemming.stem_word("pelabuhan"), Some("labuh"));

        // rule 33 : peCerV -> per-erV where C != {r|w|y|l|m|n}
        // TODO : find the examples

        // rule 34 : peCP -> pe-CP where C != {r|w|y|l|m|n} and P != 'er'
        assert_eq!(stemming.stem_word("petarung"), Some("tarung"));

        // CS additional rules

        // rule 35 : terC1erC2 -> ter-C1erC2 where C1 != 'r'
        assert_eq!(stemming.stem_word("terpercaya"), Some("percaya"));

        // rule 36 : peC1erC2 -> pe-C1erC2 where C1 != {r|w|y|l|m|n}
        assert_eq!(stemming.stem_word("pekerja"), Some("kerja"));
        assert_eq!(stemming.stem_word("peserta"), Some("serta"));

        // CS modify rule 12
        assert_eq!(stemming.stem_word("mempengaruhi"), Some("pengaruh"));

        // CS modify rule 16
        assert_eq!(stemming.stem_word("mengkritik"), Some("kritik"));

        // CS adjusting rule precedence
        assert_eq!(stemming.stem_word("bersekolah"), Some("sekolah"));
        assert_eq!(stemming.stem_word("bertahan"), Some("tahan"));
        assert_eq!(stemming.stem_word("mencapai"), Some("capai"));
        assert_eq!(stemming.stem_word("dimulai"), Some("mulai"));
        assert_eq!(stemming.stem_word("petani"), Some("tani"));
        assert_eq!(stemming.stem_word("terabaikan"), Some("abai"));

        // ECS
        assert_eq!(stemming.stem_word("mensyaratkan"), Some("syarat"));
        assert_eq!(stemming.stem_word("mensyukuri"), Some("syukur"));
        assert_eq!(stemming.stem_word("mengebom"), Some("bom"));
        assert_eq!(stemming.stem_word("mempromosikan"), Some("promosi"));
        assert_eq!(stemming.stem_word("memproteksi"), Some("proteksi"));
        assert_eq!(stemming.stem_word("memprediksi"), Some("prediksi"));
        assert_eq!(stemming.stem_word("pengkajian"), Some("kaji"));
        assert_eq!(stemming.stem_word("pengebom"), Some("bom"));

        // ECS loop pengembalian akhiran
        assert_eq!(stemming.stem_word("bersembunyi"), Some("sembunyi"));
        assert_eq!(stemming.stem_word("bersembunyilah"), Some("sembunyi"));
        assert_eq!(stemming.stem_word("pelanggan"), Some("langgan"));
        assert_eq!(stemming.stem_word("pelaku"), Some("laku"));
        assert_eq!(stemming.stem_word("pelangganmukah"), Some("langgan"));
        assert_eq!(stemming.stem_word("pelakunyalah"), Some("laku"));

        assert_eq!(stemming.stem_word("perbaikan"), Some("baik"));
        assert_eq!(stemming.stem_word("kebaikannya"), Some("baik"));
        assert_eq!(stemming.stem_word("bisikan"), Some("bisik"));
        assert_eq!(stemming.stem_word("menerangi"), Some("terang"));
        assert_eq!(stemming.stem_word("berimanlah"), Some("iman"));

        assert_eq!(stemming.stem_word("memuaskan"), Some("puas"));
        assert_eq!(stemming.stem_word("berpelanggan"), Some("langgan"));
        assert_eq!(stemming.stem_word("bermakanan"), Some("makan"));

        // CC (Modified ECS)
        assert_eq!(stemming.stem_word("menyala"), Some("nyala"));
        assert_eq!(stemming.stem_word("menyanyikan"), Some("nyanyi"));
        assert_eq!(stemming.stem_word("menyatakannya"), Some("nyata"));

        assert_eq!(stemming.stem_word("penyanyi"), Some("nyanyi"));
        assert_eq!(stemming.stem_word("penyawaan"), Some("nyawa"));

        // CC infix
        assert_eq!(stemming.stem_word("rerata"), Some("rata"));
        assert_eq!(stemming.stem_word("lelembut"), Some("lembut"));
        assert_eq!(stemming.stem_word("lemigas"), Some("ligas"));
        assert_eq!(stemming.stem_word("kinerja"), Some("kerja"));

        // plurals
        assert_eq!(stemming.stem_word("buku-buku"), Some("buku"));
        assert_eq!(stemming.stem_word("berbalas-balasan"), Some("balas"));
        assert_eq!(stemming.stem_word("bolak-balik"), Some("bolak-balik"));

        // combination of prefix + suffix
        assert_eq!(stemming.stem_word("bertebaran"), Some("tebar"));
        assert_eq!(stemming.stem_word("terasingkan"), Some("asing"));
        assert_eq!(stemming.stem_word("membangunkan"), Some("bangun"));
        assert_eq!(stemming.stem_word("mencintai"), Some("cinta"));
        assert_eq!(stemming.stem_word("menduakan"), Some("dua"));
        assert_eq!(stemming.stem_word("menjauhi"), Some("jauh"));
        assert_eq!(stemming.stem_word("menggilai"), Some("gila"));
        assert_eq!(stemming.stem_word("pembangunan"), Some("bangun"));

        // recursively remove prefix
        assert_eq!(stemming.stem_word("memberdayakan"), Some("daya"));
        assert_eq!(stemming.stem_word("persemakmuran"), Some("makmur"));
        assert_eq!(stemming.stem_word("keberuntunganmu"), Some("untung"));
        assert_eq!(stemming.stem_word("kesepersepuluhnya"), Some("puluh"));

        // issues
        assert_eq!(stemming.stem_word("perekonomian"), Some("ekonomi"));
        assert_eq!(stemming.stem_word("menahan"), Some("tahan"));

        // failed on other method / algorithm but we should succeed
        assert_eq!(stemming.stem_word("peranan"), Some("peran"));
        assert_eq!(stemming.stem_word("memberikan"), Some("beri"));
        assert_eq!(stemming.stem_word("medannya"), Some("medan"));

        // TODO:
        assert_eq!(stemming.stem_word("sebagai"), Some("bagai"));
        assert_eq!(stemming.stem_word("bagian"), Some("bagi"));
        assert_eq!(stemming.stem_word("berbadan"), Some("badan"));
        assert_eq!(stemming.stem_word("abdullah"), Some("abdullah"));

        // adopted foreign suffixes
        assert_eq!(stemming.stem_word("budayawan"), Some("budaya"));
        assert_eq!(stemming.stem_word("karyawati"), Some("karya"));
        assert_eq!(stemming.stem_word("idealis"), Some("ideal"));
        assert_eq!(stemming.stem_word("idealisme"), Some("ideal"));
        assert_eq!(stemming.stem_word("finalisasi"), Some("final"));

        // sastrawi additional rules
        assert_eq!(stemming.stem_word("penstabilan"), Some("stabil"));
        assert_eq!(stemming.stem_word("pentranskripsi"), Some("transkripsi"));

        assert_eq!(stemming.stem_word("mentaati"), Some("taat"));
        assert_eq!(stemming.stem_word("meniru-nirukan"), Some("tiru"));
        assert_eq!(stemming.stem_word("menyepak-nyepak"), Some("sepak"));

        assert_eq!(stemming.stem_word("melewati"), Some("lewat"));
        assert_eq!(stemming.stem_word("menganga"), Some("nganga"));

        assert_eq!(stemming.stem_word("kupukul"), Some("pukul"));
        assert_eq!(stemming.stem_word("kauhajar"), Some("hajar"));

        assert_eq!(stemming.stem_word("kuasa-mu"), Some("kuasa"));
        assert_eq!(
            stemming.stem_word("malaikat-malaikat-nya"),
            Some("malaikat")
        );
        assert_eq!(stemming.stem_word("nikmat-ku"), Some("nikmat"));
        assert_eq!(stemming.stem_word("allah-lah"), Some("allah"));
    }
}
