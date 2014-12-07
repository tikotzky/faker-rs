use super::image::Image;
use super::lorem::Lorem;
use super::name::Name;
use super::number::Number;
use super::locale::{Locale,en};

pub struct Faker {
    pub image: Image,
    pub lorem: Lorem,
    pub name: Name,
    pub number: Number
}

impl Faker {
    pub fn new(locale: &str) -> Faker {

        let strings = match locale {
            "en" => en::load(),
            _ => en::load()
        };

        Faker{
            image:  Image::new  (strings.clone()),
            lorem:  Lorem::new  (strings.clone()),
            name:   Name::new   (strings.clone()) ,
            number: Number::new (strings.clone())
        }
    }
}