use super::helpers;
use std::fmt;

pub struct Image {
	categories: Vec<String>
}

impl Image {
	fn new(categories: Vec<String>) -> Image {
		Image {categories: categories}
	}

	fn image(&self, width: int, height: int) -> String {
		self.category(width, height, helpers::array_element(self.categories.as_slice()).as_slice())
	}

	fn category(&self, width: int, height: int, category: &str) -> String {
		format!("http://lorempixel.com/{}/{}/{}", width, height, category)
	}

	fn avatar(&self, slug: &str, size: &str, format: &str) -> String{
		format!("http://robohash.org/#{}.#{}?size=#{}", slug, format, size)
	}

}

#[cfg(test)]
mod tests;