#[test]
fn test_words(){
	let words = super::words(10);
	assert!(words.len() == 10);
}

#[test]
fn test_sentence(){
	let sentence = super::sentence(2, 5);
	let count = sentence.split(' ').count();
	assert!(count >= 2 && count <= 7);
}

#[test]
fn test_paragraph(){

}

#[test]
fn test_paragraphs(){
	
}