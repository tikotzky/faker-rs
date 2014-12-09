#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;
extern crate faker;
use faker::Faker;


#[test]
fn test_number() {
	let faker = Faker::new("en");
	for _ in range(0u, 1000) {
		let matched = regex!(r"\d{10}").is_match(faker.number.number(10).as_slice());
		assert!(matched);
	}
}

#[test]
fn test_digit() {
	let faker = Faker::new("en");
	for _ in range(0u, 1000) {
		let matched = regex!(r"\d{1}").is_match(faker.number.digit().as_slice());
		assert!(matched);
	}
}