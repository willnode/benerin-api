use rustpostal::address::AddressParserResponse;
use rustpostal::LibModules;
use rustpostal::address;

pub struct Postal {
    postal: LibModules,
}

impl Postal {
    pub fn new() -> Postal {
        let postal = LibModules::Address;

        postal.setup().unwrap();
        Postal {
            postal,
        }
    }

    pub fn solve(&self, s: &str) -> AddressParserResponse {
    
        let address = "St Johns Centre, Rope Walk, Bedford, Bedfordshire, MK42 0XE, United Kingdom";
        address::parse_address(s, Some("id"), Some("id")).unwrap()
    
        // for (token, label) in &labeled_tokens {
        //     println!("{}: {}", token, label);
        // }
    
        // expand::expand_address_with_options(address, Some(["id"].iter()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p = Postal::new();
        let labeled_tokens = p.solve("Jl. Hayamwuruk");
        for (token, label) in &labeled_tokens {
            println!("{}: {}", token, label);
        }
    }
}
