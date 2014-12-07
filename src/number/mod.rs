use super::helpers;

struct Number;

impl Number {

	fn new() -> Number{
		Number
	}

	fn digit(&self) -> String {
		helpers::number_in_range(0u, 9).to_string()
	}

	fn number(&self, num: int) -> String {
	 	range(0, num).map(|x| x.to_string()).collect::<Vec<String>>().connect("")
	}

}

#[cfg(test)]
mod tests;
