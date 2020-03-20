use super::helpers;

/// ## Number
///
/// ```
/// use faker::Faker;
/// let faker = Faker::new("en");
///
/// faker.number.number(10); // "1968353479"
/// faker.number.digit(); // "1"
/// ```
pub struct Number;

impl Number {
    pub fn new() -> Number {
        Number
    }

    pub fn digit(&self) -> String {
        helpers::number_in_range(0u32, 9).to_string()
    }

    pub fn number(&self, num: u32) -> String {
        (0..num)
            .map(|_| self.digit().to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use faker::Faker;

    #[test]
    fn test_number() {
        let faker = Faker::new("en");
        let regex = Regex::new(r"\d{10}").unwrap();
        for _ in 0..1000 {
            let matched = regex.is_match(&faker.number.number(10));
            assert!(matched);
        }
    }

    #[test]
    fn test_digit() {
        let faker = Faker::new("en");
        let regex = Regex::new(r"\d{1}").unwrap();
        for _ in 0..1000 {
            let matched = regex.is_match(&faker.number.digit());
            assert!(matched);
        }
    }
}
