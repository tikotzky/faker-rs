use super::locale::Locale;
use super::helpers;
use std::fmt;

pub struct Image {
	categories: Vec<&'static str>
}

impl Image {
	pub fn new(locale: Locale) -> Image {
		Image {categories: locale.imageCategories}
	}

	pub fn image(&self, width: int, height: int) -> String {
		self.category(width, height, helpers::array_element(self.categories.as_slice()).as_slice())
	}

	pub fn category(&self, width: int, height: int, category: &str) -> String {
		format!("http://lorempixel.com/{}/{}/{}", width, height, category)
	}

	pub fn avatar(&self, width: int, height: int, slug: &str, format: &str) -> String{
		format!("http://robohash.org/#{}.#{}?size=#{}x{}", slug, format, width, height)
	}

}

#[cfg(test)]
mod tests;