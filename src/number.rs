use super::helpers::Helpers;

pub struct Number {
    helpers: Helpers
}

impl Number {

	pub fn new() -> Number{
		Number {
            helpers: Helpers
        }
	}

	pub fn digit(&self) -> String {
		self.helpers.number_in_range(0u, 9).to_string()
	}

	pub fn number(&self, num: int) -> String {
	 	range(0, num).map(|_| self.digit().to_string()).collect::<Vec<String>>().connect("")
	}

}
