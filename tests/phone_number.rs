#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;
extern crate faker;
use faker::Faker;


#[test]
fn test_phone_number_format() {
	let faker = Faker::new("en");
	for _ in range(0u, 1000) {
		let num = faker.phone_number.phone_number_format("N");
		assert!(regex!(r"[2-9]").is_match(num.as_slice()));

		let num = faker.phone_number.phone_number_format("Z");
		assert!(regex!(r"[1-9]").is_match(num.as_slice()));	

		let num = faker.phone_number.phone_number_format("X");
		assert!(regex!(r"[0-9]").is_match(num.as_slice()));			
	}

}