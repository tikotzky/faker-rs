#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;
extern crate faker;
use faker::Faker;


#[test]
fn test_first_name(){
	let faker = Faker::new("en");
	let name = faker.name.first_name();
	let matched = regex!(r"^\w+").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_last_name(){
	let faker = Faker::new("en");
	let name = faker.name.last_name();
	let matched = regex!(r"^\w+").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_name(){
	let faker = Faker::new("en");
	let name = faker.name.full_name();
	let matched = regex!(r"^(\w+\.? ?){2,3}$").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_prefix(){
	let faker = Faker::new("en");
	let name = faker.name.prefix();
	let matched = regex!(r"^[A-Z][a-z]+\.?$").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_suffix(){
	let faker = Faker::new("en");
	let name = faker.name.suffix();
	let matched = regex!(r"^[A-Z][A-Za-z]*\.?$").is_match(name.as_slice());
	assert!(matched);
}