use super::helpers::Helpers;
use super::image::Image;
use super::lorem::Lorem;
use super::name::Name;
use super::number::Number;
use super::phone_number::PhoneNumber;
use super::address::Address;
use super::locale::en;

pub struct Faker {
    pub helpers: Helpers,
    pub image: Image,
    pub lorem: Lorem,
    pub name: Name,
    pub number: Number,
    pub phone_number: PhoneNumber,
    pub address: Address
}

impl Faker {

    pub fn new(locale: &str) -> Faker {

        let strings = match locale {
            "en" => en::load(),
            _ => en::load()
        };

        Faker{
            helpers     :   Helpers::new     (),
            image       :   Image::new       (strings.clone()),
            lorem       :   Lorem::new       (strings.clone()),
            name        :   Name::new        (strings.clone()),
            number      :   Number::new      (),
            phone_number:   PhoneNumber::new (strings.clone()),
            address     :   Address::new     (strings.clone())
        }
    }
    
}