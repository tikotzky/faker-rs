use super::helpers;
use super::random;

// TODO add valid area codes.

pub fn phone_number() -> String{
    let format = phone_formats();
	helpers::replace_sym_with_number(format)
}

pub fn phone_number_format(format: &str) -> String{
    helpers::replace_sym_with_number(format.to_string())
}

pub fn phone_formats() -> String {
    random::array_element(&FORMATS).to_string()
}

static FORMATS: [&'static str, ..16] = [
    "###-###-####",
    "(###)###-####",
    "###.###.####",
    "1-###-###-####",
    "###-###-#### x###",
    "(###)###-#### x###",
    "1-###-###-#### x###",
    "###.###.#### x###",
    "###-###-#### x####",
    "(###)###-#### x####",
    "1-###-###-#### x####",
    "###.###.#### x####",
    "###-###-#### x#####",
    "(###)###-#### x#####",
    "1-###-###-#### x#####",
    "###.###.#### x#####"
  ];

mod tests;