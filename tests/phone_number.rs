extern crate regex;
extern crate faker;

use regex::Regex;
use faker::Faker;

#[test]
fn test_phone_number_format() {
	let faker = Faker::new("en");
	for _ in 0u32..1000 {
		let num = faker.phone_number.phone_number_format("N");
		assert!(Regex::new(r"[2-9]").unwrap().is_match(&num));

		let num = faker.phone_number.phone_number_format("Z");
		assert!(Regex::new(r"[1-9]").unwrap().is_match(&num));

		let num = faker.phone_number.phone_number_format("X");
		assert!(Regex::new(r"[0-9]").unwrap().is_match(&num));
	}

}
