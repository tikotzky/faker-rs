use super::helpers;
use super::locale::Locale;

/// Fake Addresses
///
/// ```
/// use faker::Faker;
/// let faker = Faker::new("en");
///
/// faker.address.city_prefix();       // => "West"
/// faker.address.city_suffix();       // => "Mouth"
/// faker.address.street_suffix();     // => "Landing"
/// faker.address.state();             // => "New Jersey"
/// faker.address.time_zone();         // => "Europe/Helsinki"
/// faker.address.building_number();   // => "853"
/// faker.address.zip();               // => "00531"
/// faker.address.secondary_address(); // => "Apt. 329"
/// faker.address.city();              // => "North Rasheedview"
/// faker.address.street_name();       // => "Haag Station"
/// faker.address.street_address();    // => "2814 Shannon Roads"
/// faker.address.state_abbr();        // => "AR"
/// faker.address.country();           // => "Cyprus"
/// faker.address.latitude();          // => "87.099724"
/// faker.address.longitude();         // => "-58.324116"
/// ```
pub struct Address {
    building_number: Vec<&'static str>,
    city_prefix: Vec<&'static str>,
    city_suffix: Vec<&'static str>,
    street_suffix: Vec<&'static str>,
    secondary_address: Vec<&'static str>,
    first_names: Vec<&'static str>,
    last_names: Vec<&'static str>,
    state: Vec<&'static str>,
    zip: Vec<&'static str>,
    time_zone: Vec<&'static str>,
    state_abbr: Vec<&'static str>,
    country: Vec<&'static str>,
}

impl Address {
    pub fn new(locale: Locale) -> Address {
        Address {
            building_number: locale.building_number,
            city_prefix: locale.city_prefix,
            city_suffix: locale.city_suffix,
            street_suffix: locale.street_suffix,
            secondary_address: locale.secondary_address,
            first_names: locale.name_first,
            last_names: locale.name_last,
            state: locale.state,
            zip: locale.zip,
            time_zone: locale.time_zone,
            state_abbr: locale.state_abbr,
            country: locale.country,
        }
    }

    fn first_name(&self) -> String {
        helpers::array_element(&self.first_names).to_string()
    }

    fn last_name(&self) -> String {
        helpers::array_element(&self.last_names).to_string()
    }

    pub fn city_prefix(&self) -> String {
        helpers::array_element(&self.city_prefix).to_string()
    }

    pub fn city_suffix(&self) -> String {
        helpers::sentence_case(helpers::array_element(&self.city_suffix).to_string())
    }

    fn city_suffix_lower(&self) -> String {
        helpers::lowercase(self.city_suffix())
    }

    pub fn street_suffix(&self) -> String {
        helpers::array_element(&self.street_suffix).to_string()
    }

    pub fn state(&self) -> String {
        helpers::array_element(&self.state).to_string()
    }

    pub fn country(&self) -> String {
        helpers::array_element(&self.country).to_string()
    }

    pub fn time_zone(&self) -> String {
        helpers::array_element(&self.time_zone).to_string()
    }

    pub fn state_abbr(&self) -> String {
        helpers::array_element(&self.state_abbr).to_string()
    }

    pub fn building_number(&self) -> String {
        let format = helpers::array_element(&self.building_number).to_string();
        helpers::replace_sym_with_number(format)
    }

    pub fn zip(&self) -> String {
        let format = helpers::array_element(&self.zip).to_string();
        helpers::replace_sym_with_number(format)
    }

    pub fn secondary_address(&self) -> String {
        let format = helpers::array_element(&self.secondary_address).to_string();
        helpers::replace_sym_with_number(format)
    }

    pub fn city(&self) -> String {
        match helpers::number_in_range::<i32>(0, 3) {
            0 => format!(
                "{} {}{}",
                self.city_prefix(),
                self.first_name(),
                self.city_suffix_lower()
            ),
            1 => format!("{} {}", self.city_prefix(), self.first_name()),
            2 => format!("{}{}", self.first_name(), self.city_suffix_lower()),
            _ => format!("{}{}", self.last_name(), self.city_suffix_lower()),
        }
    }

    pub fn street_name(&self) -> String {
        match helpers::number_in_range::<i32>(0, 1) {
            0 => format!("{} {}", self.first_name(), self.street_suffix()),
            _ => format!("{} {}", self.last_name(), self.street_suffix()),
        }
    }

    pub fn street_address(&self) -> String {
        format!("{} {}", self.building_number(), self.street_name())
    }

    pub fn latitude(&self) -> String {
        ((helpers::number::<f64>() * 180f64) - 90f64).to_string()
    }

    pub fn longitude(&self) -> String {
        ((helpers::number::<f64>() * 360f64) - 180f64).to_string()
    }
}
