use super::helpers;

struct Name {
    first_names: Vec<String>, 
    last_names: Vec<String>, 
    prefix: Vec<String>, 
    suffix: Vec<String>
}

impl Name {
    fn new(first_names: Vec<String>, last_names: Vec<String>, prefix: Vec<String>, suffix: Vec<String>) -> Name {
        Name {first_names: first_names, last_names: last_names, prefix: prefix, suffix: suffix}
    }

    fn first_name(&self) -> String {
        helpers::array_element(self.first_names.as_slice()).to_string()
    }

    fn last_name(&self)  -> String {
        helpers::array_element(self.last_names.as_slice()).to_string()
    }

    fn prefix(&self)     -> String {
        helpers::array_element(self.prefix.as_slice()).to_string()
    }

    fn suffix(&self)     -> String {
        helpers::array_element(self.suffix.as_slice()).to_string()
    }

    fn find_name(&self)  -> String {
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