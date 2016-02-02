extern crate regex;
extern crate faker;

use regex::Regex;
use faker::Faker;

#[test]
fn test_image() {
	let faker = Faker::new("en");
	let matched = Regex::new(r"http://lorempixel.com/100/100/\w+").unwrap().is_match(&faker.image.image(100, 100));
	assert!(matched);
}

#[test]
fn test_category() {
	let faker = Faker::new("en");
	let matched = Regex::new(r"http://lorempixel.com/100/100/cats").unwrap().is_match(&faker.image.category(100, 100, "cats"));
    assert!(matched);
}

#[test]
fn test_avatar() {
	let faker = Faker::new("en");
    assert_eq!(&faker.image.avatar(50, 50, "tikotzky", "bmp"), "http://robohash.org/#tikotzky.#bmp?size=#50x50");
}
