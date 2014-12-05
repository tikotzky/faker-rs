#[test]
fn test_number() {
	let num = super::Number::new();
	for _ in range(0u, 1000) {
		let matched = regex!(r"\d{10}").is_match(num.number(10).as_slice());
		assert!(matched);
	}
}


#[test]
fn test_digit() {
	let num = super::Number::new();
	for _ in range(0u, 1000) {
		let matched = regex!(r"\d{1}").is_match(num.digit().as_slice());
		let dig = num.digit();
		assert!(matched);
	}
}