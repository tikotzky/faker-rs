pub struct Faker {
    pub lorem: super::lorem::Lorem
}

impl Faker {
    pub fn new(locale: &str) -> Faker {

        let strings = match locale {
            "en" => super::locale::en::load(),
            _ => super::locale::en::load()
        };

        Faker{
            lorem: super::lorem::Lorem::new(strings)
        }
    }
}