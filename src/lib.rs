extern crate rand;
extern crate regex;

pub use faker::Faker;

pub trait Fake {
    fn Fake(locale: locale::Locale) -> Self;
}

mod faker;
mod locale;
mod helpers;
pub mod image;
pub mod lorem;
pub mod name;
pub mod number;
pub mod phone_number;
pub mod address;
