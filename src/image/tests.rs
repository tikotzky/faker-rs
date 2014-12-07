#[test]
fn test_image() {
	let image = new_image();
	let matched = regex!(r"http://lorempixel.com/100/100/\w+").is_match(image.image(100i, 100i).as_slice());
	assert!(matched);
}

fn test_category() {
	let image = new_image();
	let matched = regex!(r"http://lorempixel.com/100/100/cats").is_match(image.category(100i,100i,"cats").as_slice());
}

fn test_avatar() {
	let image = new_image();
	let matched = regex!(r"http://robohash.org/my-own-slug.bmp\?size=50x50").is_match(image.avatar("my-own-slug", "50x50", "bmp").as_slice());
}

fn new_image() -> super::Image{
	super::Image::new(CATEGORIES.to_vec().iter().map(|x| x.to_string()).collect())
}

static CATEGORIES: [&'static str, ..13] = ["abstract", "animals", "business", "cats", "city", "food", "nightlife", "fashion", "people", "nature", "sports", "technics", "transport"];