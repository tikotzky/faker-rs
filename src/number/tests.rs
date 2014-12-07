#[test]
fn test_number() {
	let num = new_number();
	for _ in range(0u, 1000) {
		let matched = regex!(r"\d{10}").is_match(num.number(10).as_slice());
		assert!(matched);
	}
}


#[test]
fn test_digit() {
	let num = new_number();
	for _ in range(0u, 1000) {
		let matched = regex!(r"\d{1}").is_match(num.digit().as_slice());
		assert!(matched);
	}
}

fn new_number() -> super::Number {
    super::Number{
        helpers: super::super::helpers::Helpers
    }
}
