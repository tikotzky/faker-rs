#![feature(phase)]
extern crate regex;
#[phase(plugin)]
extern crate regex_macros;
extern crate faker;
use faker::Faker;

#[test]
fn test_image() {
	let faker = Faker::new("en");
	let matched = regex!(r"http://lorempixel.com/100/100/\w+").is_match(faker.image.image(100i, 100i).as_slice());
	assert!(matched);
}

#[test]
fn test_category() {
	let faker = Faker::new("en");
	let matched = regex!(r"http://lorempixel.com/100/100/cats").is_match(faker.image.category(100i,100i,"cats").as_slice());
    assert!(matched);
}

#[test]
fn test_avatar() {
	let faker = Faker::new("en");
    assert_eq!(faker.image.avatar(50, 50, "tikotzky", "bmp").as_slice(), "http://robohash.org/#tikotzky.#bmp?size=#50x50");
}
