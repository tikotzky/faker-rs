#[test]
#[should_fail]
fn test_phone_number(){
	let phone_number = super::phone_number();
	assert!(regex!(r"#").is_match(phone_number.as_slice()));	
}