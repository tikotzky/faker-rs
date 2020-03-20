use super::address::Address;
use super::image::Image;
use super::locale::Locale;
use super::lorem::Lorem;
use super::name::Name;
use super::number::Number;
use super::phone_number::PhoneNumber;

/// **Main** structure for creating fake data
pub struct Faker {
    pub image: Image,
    pub lorem: Lorem,
    pub name: Name,
    pub number: Number,
    pub phone_number: PhoneNumber,
    pub address: Address,
}

impl Faker {
    pub fn new(locale: &str) -> Faker {
        let db = Locale::load(locale);

        Faker {
            number: Number::new(),
            image: Image::new(db.clone()),
            lorem: Lorem::new(db.clone()),
            name: Name::new(db.clone()),
            phone_number: PhoneNumber::new(db.clone()),
            address: Address::new(db.clone()),
        }
    }
}
