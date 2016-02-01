use super::locale::Locale;
use super::helpers::Helpers;

pub struct Name {
    first_names: Vec<&'static str>,
    last_names: Vec<&'static str>,
    prefix: Vec<&'static str>,
    suffix: Vec<&'static str>,
    title: Vec<&'static str>,
    helpers: Helpers
}

impl Name {

    pub fn new(locale: Locale) -> Name {
        Name {
            first_names: locale.name_first,
            last_names: locale.name_last,
            prefix: locale.name_prefix,
            suffix: locale.name_suffix,
            title: locale.name_title,
            helpers: Helpers
        }
    }

    pub fn title(&self) -> String {
        self.helpers.array_element(&self.title).to_string()
    }

    pub fn first_name(&self) -> String {
        self.helpers.array_element(&self.first_names).to_string()
    }

    pub fn last_name(&self)  -> String {
        self.helpers.array_element(&self.last_names).to_string()
    }

    pub fn prefix(&self)     -> String {
        self.helpers.array_element(&self.prefix).to_string()
    }

    pub fn suffix(&self)     -> String {
        self.helpers.array_element(&self.suffix).to_string()
    }

    pub fn full_name(&self)  -> String {
        match self.helpers.number_in_range::<i32>(0, 8) {
            0 => format!("{} {} {}", self.prefix(), self.first_name(), self.last_name()),
            1 => format!("{} {} {}", self.first_name(), self.last_name(), self.suffix()),
            _ => format!("{} {}", self.first_name(), self.last_name())
        }
    }

}
