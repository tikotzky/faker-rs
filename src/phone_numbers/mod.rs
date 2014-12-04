use super::helpers;
use super::random;

// TODO add valid area codes.

pub fn phone_number() -> String{
    phone_number_format(phone_formats().as_slice())
}

pub fn phone_number_format(format: &str) -> String{
    helpers::replace_sym_with_number(format.to_string())
}

pub fn phone_formats() -> String {
    random::array_element(&FORMATS).to_string()
}

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

mod tests;