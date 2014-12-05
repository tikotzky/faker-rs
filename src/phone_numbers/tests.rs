static FORMATS: [&'static str, ..16] = [
    "NXX-NXX-XXXX",
    "(NXX)NXX-XXXX",
    "NXX.NXX.XXXX",
    "1-NXX-NXX-XXXX",
    "NXX-NXX-XXXX xNXX",
    "(NXX)NXX-XXXX xNXX",
    "1-NXX-NXX-XXXX xNXX",
    "NXX.NXX.XXXX xNXX",
    "NXX-NXX-XXXX xNXXX",
    "(NXX)NXX-XXXX xNXXX",
    "1-NXX-NXX-XXXX xNXXX",
    "NXX.NXX.XXXX xNXXX",
    "NXX-NXX-XXXX xNXXXX",
    "(NXX)NXX-NXX xNXXXX",
    "1-NXX-NXX-XXXX xNXXXX",
    "NXX.NXX.XXXX xNXXXX"
  ];


#[test]
fn test_phone_number_format() {
	let phone_number = super::PhoneNumber::new(FORMATS.to_vec().iter().map(|x| x.to_string()).collect());
	for _ in range(0u, 1000) {
		let num = phone_number.phone_number_format("N");
		assert!(regex!(r"[2-9]").is_match(num.as_slice()));

		let num = phone_number.phone_number_format("Z");
		assert!(regex!(r"[1-9]").is_match(num.as_slice()));	

		let num = phone_number.phone_number_format("X");
		assert!(regex!(r"[0-9]").is_match(num.as_slice()));			
	}

}

