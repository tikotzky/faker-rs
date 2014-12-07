use super::locale::Locale;
use super::helpers;

pub struct Name {
    first_names: Vec<&'static str>, 
    last_names: Vec<&'static str>, 
    prefix: Vec<&'static str>, 
    suffix: Vec<&'static str>,
    title: Vec<&'static str>
}

impl Name {
    pub fn new(locale: Locale) -> Name {
        Name {
            first_names: locale.name_first, 
            last_names: locale.name_last, 
            prefix: locale.name_prefix, 
            suffix: locale.name_suffix, 
            title: locale.name_title
        }
    }

    pub fn title(&self) -> String {
        helpers::array_element(self.title.as_slice()).to_string()
    }

    pub fn first_name(&self) -> String {
        helpers::array_element(self.first_names.as_slice()).to_string()
    }

    pub fn last_name(&self)  -> String {
        helpers::array_element(self.last_names.as_slice()).to_string()
    }

    pub fn prefix(&self)     -> String {
        helpers::array_element(self.prefix.as_slice()).to_string()
    }

    pub fn suffix(&self)     -> String {
        helpers::array_element(self.suffix.as_slice()).to_string()
    }

    pub fn name(&self)  -> String {
        let name = self.first_name() + " " + self.last_name();
        match helpers::number_in_range::<int>(0, 8) {
            0 => self.prefix() + " " + name,
            1 => name + " " + self.suffix(),
            _ => name
        }
    }
}

#[cfg(test)]
mod tests;