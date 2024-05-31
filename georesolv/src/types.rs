use rustpostal::address::AddressParserResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressEntity {
    #[serde(skip_serializing_if = "str_is_empty")]
    pub house: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub category: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub near: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub house_number: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub road: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub unit: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub level: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub staircase: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub entrance: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub po_box: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub postcode: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub suburb: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub city_district: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub city: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub island: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub state_district: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub state: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub country_region: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub country: String,
    #[serde(skip_serializing_if = "str_is_empty")]
    pub world_region: String,
}

impl AddressEntity {
    pub fn empty() -> AddressEntity {
        AddressEntity {
            house: "".to_owned(),
            category: "".to_owned(),
            near: "".to_owned(),
            house_number: "".to_owned(),
            road: "".to_owned(),
            unit: "".to_owned(),
            level: "".to_owned(),
            staircase: "".to_owned(),
            entrance: "".to_owned(),
            po_box: "".to_owned(),
            postcode: "".to_owned(),
            suburb: "".to_owned(),
            city_district: "".to_owned(),
            city: "".to_owned(),
            island: "".to_owned(),
            state_district: "".to_owned(),
            state: "".to_owned(),
            country_region: "".to_owned(),
            country: "".to_owned(),
            world_region: "".to_owned(),
        }
    }

    pub fn from_parsed(parsed: AddressParserResponse) -> AddressEntity {
        let r = AddressEntity::empty();
        for (token, label) in &labeled_tokens {
            match token {
                "house" => r.house = label,
                "category" => r.category = label,
                "near" => r.near = label,
                "house_number" => r.house_number = label,
                "road" => r.road = label,
                "unit" => r.unit = label,
                "level" => r.level = label,
                "staircase" => r.staircase = label,
                "entrance" => r.entrance = label,
                "po_box" => r.po_box = label,
                "postcode" => r.postcode = label,
                "suburb" => r.suburb = label,
                "city_district" => r.city_district = label,
                "city" => r.city = label,
                "island" => r.island = label,
                "state_district" => r.state_district = label,
                "state" => r.state = label,
                "country_region" => r.country_region = label,
                "country" => r.country = label,
                "world_region" => r.world_region = label,
            }
        }
        r
    }
}


fn metadata_is_empty(str: &String) -> bool {
   str == ""
}
