use super::helpers;
use super::locale::Locale;

/// ## Phone Number
///
/// ```
/// use faker::Faker;
/// let faker = Faker::new("en");
///
/// //returns a random number in a random format
/// faker.phone_number.phone_number(); // "397.693.1309"
/// //takes a format and returns a number
/// faker.phone_number.phone_number_format("1-NXX-NXX-XXXX"); // "1-397-693-1309"
/// //returns a random phone format
/// faker.phone_number.phone_formats(); // 1-NXX-NXX-XXXX"
/// ```
/// ### Formating
/// Phone numbers may be in any of the following formats:
/// * "NXX-NXX-XXXX"
/// * "(NXX)NXX-XXXX"
/// * "NXX.NXX.XXXX"
/// * "1-NXX-NXX-XXXX"
/// * "NXX-NXX-XXXX xNXX"
/// * "(NXX)NXX-XXXX xNXX"
/// * "1-NXX-NXX-XXXX xNXX"
/// * "NXX.NXX.XXXX xNXX"
/// * "NXX-NXX-XXXX xNXXX"
/// * "(NXX)NXX-XXXX xNXXX"
/// * "1-NXX-NXX-XXXX xNXXX"
/// * "NXX.NXX.XXXX xNXXX"
/// * "NXX-NXX-XXXX xNXXXX"
/// * "(NXX)NXX-NXX xNXXXX"
/// * "1-NXX-NXX-XXXX xNXXXX"
/// * "NXX.NXX.XXXX xNXXXX"
///
/// ##### This format gets replaced with the following:
/// - 'X' or '#' => a number 0-9
/// - 'Z' => a number 1-9
/// - 'N' => a number 2-9
pub struct PhoneNumber {
    formats: Vec<&'static str>,
}

impl PhoneNumber {
    pub fn new(locale: Locale) -> PhoneNumber {
        PhoneNumber {
            formats: locale.phone_formats,
        }
    }

    pub fn phone_number(&self) -> String {
        self.phone_number_format(&self.phone_formats())
    }

    pub fn phone_number_format(&self, format: &str) -> String {
        helpers::replace_sym_with_number(format.to_string())
    }

    pub fn phone_formats(&self) -> String {
        helpers::array_element(&self.formats).to_string()
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use faker::Faker;

    #[test]
    fn test_phone_number_format() {
        let faker = Faker::new("en");
        let regex_0_9 = Regex::new(r"[0-9]").unwrap();
        let regex_1_9 = Regex::new(r"[1-9]").unwrap();
        let regex_2_9 = Regex::new(r"[2-9]").unwrap();
        for _ in 0u32..1000 {
            let num = faker.phone_number.phone_number_format("N");
            assert!(regex_2_9.is_match(&num));

            let num = faker.phone_number.phone_number_format("Z");
            assert!(regex_1_9.is_match(&num));

            let num = faker.phone_number.phone_number_format("X");
            assert!(regex_0_9.is_match(&num));
        }
    }
}
