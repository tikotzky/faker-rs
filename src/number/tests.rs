#[test]
fn test_number() {
	let num = super::Number::new();
	for _ in range(0u, 1000) {
		assert_eq!(num.number(10).len(), 10);
	}
}


#[test]
fn test_digit() {
	let num = super::Number::new();
	for _ in range(0u, 1000) {
		let dig = num.digit();
		assert!(dig >= 0 && dig <= 9);
	}
}
