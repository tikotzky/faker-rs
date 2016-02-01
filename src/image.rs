use super::locale::Locale;
use super::helpers::Helpers;

pub struct Image {
	categories: Vec<&'static str>,
    helpers: Helpers
}

impl Image {

	pub fn new(locale: Locale) -> Image {
		Image {
            categories: locale.image_categories,
            helpers: Helpers
        }
	}

	pub fn image(&self, width: i32, height: i32) -> String {
		self.category(width, height, &self.helpers.array_element(&self.categories))
	}

	pub fn category(&self, width: i32, height: i32, category: &str) -> String {
		format!("http://lorempixel.com/{}/{}/{}", width, height, category)
	}

	pub fn avatar(&self, width: i32, height: i32, slug: &str, format: &str) -> String{
		format!("http://robohash.org/#{}.#{}?size=#{}x{}", slug, format, width, height)
	}

}
