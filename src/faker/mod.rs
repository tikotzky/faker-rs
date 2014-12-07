use super::lorem::Lorem;
use super::image::Image;
use super::locale::Locale;
use super::locale::{en};

pub struct Faker {
    pub lorem: Lorem,
    pub image: Image
}

impl Faker {
    pub fn new(locale: &str) -> Faker {

        let strings = match locale {
            "en" => en::load(),
            _ => en::load()
        };

        Faker{
            lorem: Lorem::new(strings.clone()),
            image: Image::new(strings.clone())
        }
    }
}