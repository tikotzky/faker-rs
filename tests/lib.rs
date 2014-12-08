#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;
extern crate faker;
use faker::Faker;

#[test]
fn test_shuffle(){
    let faker = Faker::new("en");
    static TEST_ARRAY: [&'static str, ..5] = [
        "One",
        "Two",
        "Three",
        "Four",
        "Five"
    ];

    let a = faker.helpers.shuffle(&TEST_ARRAY);
    // Test that we got back an array of the same length
    assert_eq!(a.len(), 5);
    // Test thhat at least one of the array elements are not in the original position
    // (This test can fail in the event that it "randomly" returns all elements in the original order)
    assert!(a[0] != "One" || a[1] != "Two" || a[2] != "Three" || a[3] != "Four" || a[4] != "Five");
}

#[test]
fn test_helper_number(){
    let faker = Faker::new("en");
    let a: int = faker.helpers.number();
    let b = 1i;
    assert_eq!(a/b, a);
}

#[test]
fn test_helper_number_in_range(){
    let faker = Faker::new("en");
    for _ in range(0u, 1000) {
        let a = faker.helpers.number_in_range(-3i, 42);
        assert!(a >= -3 && a <= 42);
        assert_eq!(faker.helpers.number_in_range(0i, 0), 0);
        assert_eq!(faker.helpers.number_in_range(-12i, -12), -12);
    }
    for _ in range(0u, 1000) {
        let a = faker.helpers.number_in_range(10i, 42);
        assert!(a >= 10 && a <= 42);
        assert_eq!(faker.helpers.number_in_range(0i, 0), 0);
        assert_eq!(faker.helpers.number_in_range(3_000_000u, 3_000_000), 3_000_000);
    }
}

#[test]
#[should_fail]
fn test_number_in_range_panic(){
    let faker = Faker::new("en");
    faker.helpers.number_in_range(9i, 7);
}

#[test]
fn test_array_element(){
    let faker = Faker::new("en");
    let a : [int, ..1] = [1];
    for _ in range(0u, 1000) {
        // test that we dont try to access an index that is out of bounds
        faker.helpers.array_element(&a);
    } 
}

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

#[test]
fn test_words(){
  let faker = Faker::new("en");
	let words = faker.lorem.words(5);
	assert!(words.len() == 5);
}

#[test]
fn test_sentence(){
	let faker = Faker::new("en");
	let sentence = faker.lorem.sentence(2, 5);
	let count = sentence.split(' ').count();
	assert!(count >= 2 && count <= 7);
}

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