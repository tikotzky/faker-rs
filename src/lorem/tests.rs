#[test]
fn test_words(){
    let lorem = new_lorem();
	let words = lorem.words(5);
	assert!(words.len() == 5);
}

#[test]
fn test_sentence(){
	let sentence = new_lorem().sentence(2, 5);
	let count = sentence.split(' ').count();
	assert!(count >= 2 && count <= 7);
}

#[test]
fn test_paragraph(){

}

#[test]
fn test_paragraphs(){
	
}


fn new_lorem() -> super::Lorem {
	super::Lorem{
        lorem: LOREM.to_vec()
    }
}

static LOREM: [&'static str, ..15] = [
    "lorem", 
    "ipsum", 
    "dolom",
    "alias",
    "consequatur",
    "aut",
    "perferendis",
    "sit",
    "voluptatem",
    "accusantium",
    "doloremque",
    "aperiam",
    "eaque",
    "ipsa",
    "quae"
];