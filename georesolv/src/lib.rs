use rustpostal::address;
use rustpostal::LibModules;
use types::AddressEntity;

mod types;

pub struct Postal {
    postal: LibModules,
}

impl Postal {
    pub fn new() -> Postal {
        let postal = LibModules::All;

        postal.setup().unwrap();
        Postal { postal }
    }

    pub fn parse(&self, s: &str) -> AddressEntity {
        address::parse_address(s, Some("id"), Some("id")).unwrap();
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
