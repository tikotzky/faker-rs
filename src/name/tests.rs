#[test]
fn test_first_name(){
	let name = super::first_name();
	let matched = regex!(r"^\w+").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_last_name(){
	let name = super::last_name();
	let matched = regex!(r"^\w+").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_find_name(){
	let name = super::find_name();
	let matched = regex!(r"^(\w+\.? ?){2,3}$").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_prefix(){
	let name = super::prefix();
	let matched = regex!(r"^[A-Z][a-z]+\.?$").is_match(name.as_slice());
	assert!(matched);
}

#[test]
fn test_suffix(){
	let name = super::suffix();
	let matched = regex!(r"^[A-Z][A-Za-z]*\.?$").is_match(name.as_slice());
	assert!(matched);
}