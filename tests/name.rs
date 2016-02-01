extern crate regex;
extern crate faker;

use regex::Regex;
use faker::Faker;

#[test]
fn test_first_name(){
	let faker = Faker::new("en");
	let name = faker.name.first_name();
	let matched = Regex::new(r"^\w+").unwrap().is_match(&name);
	assert!(matched);
}

#[test]
fn test_last_name(){
	let faker = Faker::new("en");
	let name = faker.name.last_name();
	let matched = Regex::new(r"^\w+").unwrap().is_match(&name);
	assert!(matched);
}

#[test]
fn test_name(){
	let faker = Faker::new("en");
	let name = faker.name.full_name();
	let matched = Regex::new(r"^(\w+\.? ?){2,3}$").unwrap().is_match(&name);
	assert!(matched);
}

#[test]
fn test_prefix(){
	let faker = Faker::new("en");
	let name = faker.name.prefix();
	let matched = Regex::new(r"^[A-Z][a-z]+\.?$").unwrap().is_match(&name);
	assert!(matched);
}

#[test]
fn test_suffix(){
	let faker = Faker::new("en");
	let name = faker.name.suffix();
	let matched = Regex::new(r"^[A-Z][A-Za-z]*\.?$").unwrap().is_match(&name);
	assert!(matched);
}
