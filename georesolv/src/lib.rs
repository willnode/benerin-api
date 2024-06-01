use rustpostal::address;
use rustpostal::LibModules;
use types::AddressEntity;

mod types;

pub struct Postal {}

impl Postal {
    pub fn new() -> Postal {
        let postal = LibModules::All;

        postal.setup().unwrap();
        Postal {}
    }

    pub fn parse(&self, s: &str) -> AddressEntity {
        let labeled_tokens = address::parse_address(s, Some("id"), Some("id")).unwrap();
        AddressEntity::from_parsed(labeled_tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p = Postal::new();
        let labeled_tokens =
            p.parse("Jl. Kapten Soebianto Djojohadikusumo, BSD, Serpong, Tangerang Selatan");
    }
}
