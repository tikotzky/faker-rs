#[test]
fn test_image() {
	let image = new_image();
	let matched = regex!(r"http://lorempixel.com/100/100/\w+").is_match(image.image(100i, 100i).as_slice());
	assert!(matched);
}

#[test]
fn test_category() {
	let image = new_image();
	let matched = regex!(r"http://lorempixel.com/100/100/cats").is_match(image.category(100i,100i,"cats").as_slice());
}

#[test]
fn test_avatar() {
	let image = new_image();
	let matched = regex!(r"http://robohash.org/my-own-slug.bmp\?size=50x50").is_match(image.avatar(50, 50, "my-own-slug", "bmp").as_slice());
}

fn new_image() -> super::Image{
	super::Image{
        categories: CATEGORIES.to_vec()
    }
}

static CATEGORIES: [&'static str, ..13] = [
    "abstract",
    "animals",
    "business",
    "cats",
    "city",
    "food",
    "nightlife",
    "fashion",
    "people",
    "nature",
    "sports",
    "technics",
    "transport"
];