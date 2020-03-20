use super::helpers;
use super::locale::Locale;

/// Fake images
///
/// ```
/// use faker::Faker;
/// let faker = Faker::new("en");
///
/// faker.image.avatar(50, 50, "my-own-slug", "bmp");
/// // => "http://robohash.org/my-own-slug.bmp?size=50x50"
/// faker.image.category(100, 100, "cats");
/// // => "http://lorempixel.com/100/100/cats"
/// faker.image.image(100, 100);
/// // => "http://lorempixel.com/100/100/business"
/// ```
///
/// ##### Available categories are:
///
/// * abstract
/// * animals
/// * business
/// * cats
/// * city
/// * food
/// * nightlife
/// * fashion
/// * people
/// * nature
/// * sports
/// * technics
/// * transport
pub struct Image {
    categories: Vec<&'static str>,
}

impl Image {
    pub fn new(locale: Locale) -> Image {
        Image {
            categories: locale.image_categories,
        }
    }

    pub fn image(&self, width: i32, height: i32) -> String {
        self.category(width, height, &helpers::array_element(&self.categories))
    }

    pub fn category(&self, width: i32, height: i32, category: &str) -> String {
        format!("http://lorempixel.com/{}/{}/{}", width, height, category)
    }

    pub fn avatar(&self, width: i32, height: i32, slug: &str, format: &str) -> String {
        format!(
            "http://robohash.org/#{}.#{}?size=#{}x{}",
            slug, format, width, height
        )
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use faker::Faker;

    #[test]
    fn test_image() {
        let faker = Faker::new("en");
        let matched = Regex::new(r"http://lorempixel.com/100/100/\w+")
            .unwrap()
            .is_match(&faker.image.image(100, 100));
        assert!(matched);
    }

    #[test]
    fn test_category() {
        let faker = Faker::new("en");
        let matched = Regex::new(r"http://lorempixel.com/100/100/cats")
            .unwrap()
            .is_match(&faker.image.category(100, 100, "cats"));
        assert!(matched);
    }

    #[test]
    fn test_avatar() {
        let faker = Faker::new("en");
        assert_eq!(
            &faker.image.avatar(50, 50, "tikotzky", "bmp"),
            "http://robohash.org/#tikotzky.#bmp?size=#50x50"
        );
    }
}

