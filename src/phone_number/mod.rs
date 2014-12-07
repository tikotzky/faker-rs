use super::locale::Locale;
use super::helpers;

pub struct PhoneNumber {
    formats: Vec<&'static str>
}

impl PhoneNumber {
    pub fn new(locale: Locale) -> PhoneNumber {
        PhoneNumber {formats: locale.phone_formats}
    }

    pub fn phone_number(&self) -> String {
        self.phone_number_format(self.phone_formats().as_slice())
    }

    pub fn phone_number_format(&self, format: &str) -> String {
        helpers::replace_sym_with_number(format.to_string())
    }

    fn phone_formats(&self) -> String {
        helpers::array_element(self.formats.as_slice()).to_string()
    }
}

#[cfg(test)]
mod tests;