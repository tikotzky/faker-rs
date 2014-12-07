use super::helpers;

struct PhoneNumber {formats: Vec<String>}

impl PhoneNumber {
    fn new(formats: Vec<String>) -> PhoneNumber {
        PhoneNumber {formats: formats}
    }

    fn phone_number(&self) -> String {
        self.phone_number_format(self.phone_formats().as_slice())
    }

    fn phone_number_format(&self, format: &str) -> String {
        helpers::replace_sym_with_number(format.to_string())
    }

    fn phone_formats(&self) -> String {
        helpers::array_element(self.formats.as_slice()).to_string()
    }
}

#[cfg(test)]
mod tests;