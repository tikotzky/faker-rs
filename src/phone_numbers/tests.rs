#[test]
fn test_phone_number_format() {
	for _ in range(0u, 1000) {
		let num = super::phone_number_format("N");
		assert!(regex!(r"[2-9]").is_match(num.as_slice()));

		let num = super::phone_number_format("Z");
		assert!(regex!(r"[1-9]").is_match(num.as_slice()));	

		let num = super::phone_number_format("X");
		assert!(regex!(r"[0-9]").is_match(num.as_slice()));			
	}

}