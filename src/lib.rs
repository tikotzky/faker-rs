pub mod random;
pub mod name;

#[test]
fn test_number(){
	let a = random::number::<int>();
	let b = 1i;
	assert_eq!(a/b, a);
}

#[test]
fn test_number_in_range(){
	for _ in range(0u, 1000) {
		let a = random::number_in_range(-3i, 42);
		assert!(a >= -3 && a < 42);
		assert_eq!(random::number_in_range(0i, 1), 0);
		assert_eq!(random::number_in_range(-12i, -11), -12);
	}

	for _ in range(0u, 1000) {
		let a = random::number_in_range(10i, 42);
		assert!(a >= 10 && a < 42);
		assert_eq!(random::number_in_range(0i, 1), 0);
		assert_eq!(random::number_in_range(3_000_000u, 3_000_001), 3_000_000);
	}
}

#[test]
#[should_fail]
fn test_number_in_range_panic(){
	random::number_in_range(9i, 7);
}
