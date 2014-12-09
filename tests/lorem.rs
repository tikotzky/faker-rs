extern crate faker;
use faker::Faker;

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