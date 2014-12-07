use super::locale::Locale;
use super::helpers;

pub struct Number;

impl Number {

	pub fn new(locale: Locale) -> Number{
		Number
	}

	pub fn digit(&self) -> String {
		helpers::number_in_range(0u, 9).to_string()
	}

	pub fn number(&self, num: int) -> String {
	 	range(0, num).map(|x| x.to_string()).collect::<Vec<String>>().connect("")
	}

}

#[cfg(test)]
mod tests;
