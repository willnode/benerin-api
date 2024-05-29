use deepsize::DeepSizeOf;
use strsim::damerau_levenshtein;
// use strsim::{damerau_levenshtein, hamming, jaro, jaro_winkler, levenshtein, osa_distance};

#[derive(Debug, Eq, PartialEq, Clone, DeepSizeOf)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DistanceAlgorithm {
    Damerau,
}

pub struct EditDistance {
    algorithm: DistanceAlgorithm,
}

impl EditDistance {
    pub fn new(distance_algorithm: DistanceAlgorithm) -> EditDistance {
        EditDistance {
            algorithm: distance_algorithm,
        }
    }

    pub fn compare(&self, string: &str, other: &str, max_distance: i32) -> i32 {
        let distance = match self.algorithm {
            DistanceAlgorithm::Damerau => damerau_levenshtein(string, other),
        };

        if distance <= max_distance as usize {
            distance as i32
        } else {
            -1
        }
    }
}
