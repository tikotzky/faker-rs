use super::locale::Locale;
use super::helpers::Helpers;

pub struct PhoneNumber {
    formats: Vec<&'static str>,
    helpers: Helpers
}

impl PhoneNumber {

    pub fn new(locale: Locale) -> PhoneNumber {
        PhoneNumber {
            formats: locale.phone_formats,
            helpers: Helpers
        }
    }

    pub fn phone_number(&self) -> String {
        self.phone_number_format(&self.phone_formats())
    }

    pub fn phone_number_format(&self, format: &str) -> String {
        self.helpers.replace_sym_with_number(format.to_string())
    }

    fn phone_formats(&self) -> String {
        self.helpers.array_element(&self.formats).to_string()
    }

}
