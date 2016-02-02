extern crate regex;
extern crate faker;

use regex::Regex;
use faker::Faker;

#[test]
fn test_number() {
	let faker = Faker::new("en");
	for _ in 0..1000 {
		let matched = Regex::new(r"\d{10}").unwrap().is_match(&faker.number.number(10));
		assert!(matched);
	}
}

#[test]
fn test_digit() {
	let faker = Faker::new("en");
	for _ in 0..1000 {
		let matched = Regex::new(r"\d{1}").unwrap().is_match(&faker.number.digit());
		assert!(matched);
	}
}
