use super::helpers;
use super::locale::Locale;

/// ### Name
///
/// ```
/// use faker::Faker;
/// let faker = Faker::new("en");
///
/// faker.name.first_name(); // "Kaci"
/// faker.name.last_name(); // "Ernser"
/// faker.name.prefix(); // "Mr."
/// faker.name.suffix(); // "IV"
/// ```
pub struct Name {
    first_names: Vec<&'static str>,
    last_names: Vec<&'static str>,
    prefix: Vec<&'static str>,
    suffix: Vec<&'static str>,
    title: Vec<&'static str>,
}

impl Name {
    pub fn new(locale: Locale) -> Name {
        Name {
            first_names: locale.name_first,
            last_names: locale.name_last,
            prefix: locale.name_prefix,
            suffix: locale.name_suffix,
            title: locale.name_title,
        }
    }

    pub fn title(&self) -> String {
        helpers::array_element(&self.title).to_string()
    }

    pub fn first_name(&self) -> String {
        helpers::array_element(&self.first_names).to_string()
    }

    pub fn last_name(&self) -> String {
        helpers::array_element(&self.last_names).to_string()
    }

    pub fn prefix(&self) -> String {
        helpers::array_element(&self.prefix).to_string()
    }

    pub fn suffix(&self) -> String {
        helpers::array_element(&self.suffix).to_string()
    }

    pub fn full_name(&self) -> String {
        match helpers::number_in_range::<i32>(0, 8) {
            0 => format!(
                "{} {} {}",
                self.prefix(),
                self.first_name(),
                self.last_name()
            ),
            1 => format!(
                "{} {} {}",
                self.first_name(),
                self.last_name(),
                self.suffix()
            ),
            _ => format!("{} {}", self.first_name(), self.last_name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use faker::Faker;

    #[test]
    fn test_first_name() {
        let faker = Faker::new("en");
        let name = faker.name.first_name();
        let matched = Regex::new(r"^\w+").unwrap().is_match(&name);
        assert!(matched);
    }

    #[test]
    fn test_last_name() {
        let faker = Faker::new("en");
        let name = faker.name.last_name();
        let matched = Regex::new(r"^\w+").unwrap().is_match(&name);
        assert!(matched);
    }

    #[test]
    fn test_full_name() {
        let faker = Faker::new("en");
        let name = faker.name.full_name();
        let matched = Regex::new(r"^(\w+\.? ?){2,3}$").unwrap().is_match(&name);
        assert!(matched);
    }

    #[test]
    fn test_prefix() {
        let faker = Faker::new("en");
        let name = faker.name.prefix();
        let matched = Regex::new(r"^[A-Z][a-z]+\.?$").unwrap().is_match(&name);
        assert!(matched);
    }

    #[test]
    fn test_suffix() {
        let faker = Faker::new("en");
        let name = faker.name.suffix();
        let matched = Regex::new(r"^[A-Z][A-Za-z]*\.?$").unwrap().is_match(&name);
        assert!(matched);
    }
}
