use super::helpers;
use super::random;

struct PhoneNumber {formats: Vec<String>}

trait PhoneNumberTrait {
    fn phone_number(&self) -> String;
    fn phone_number_format(&self, format: &str) -> String;
    fn phone_formats(&self) -> String;
}

impl PhoneNumber {
    fn new(formats: Vec<String>) -> PhoneNumber {
        PhoneNumber {formats: formats}
    }
}

impl PhoneNumberTrait for PhoneNumber {
    fn phone_number(&self) -> String {
        self.phone_number_format(self.phone_formats().as_slice())
    }

    fn phone_number_format(&self, format: &str) -> String {
        helpers::replace_sym_with_number(format.to_string())
    }

    fn phone_formats(&self) -> String {
        random::array_element(self.formats.as_slice()).to_string()
    }
}

mod tests;