extern crate rand;
#[cfg(test)]
extern crate regex;

mod faker;
pub use faker::Faker;
mod helpers;
mod image;
mod locale;
pub use image::Image;
mod lorem;
pub use lorem::Lorem;
mod name;
pub use name::Name;
mod number;
pub use number::Number;
mod phone_number;
pub use phone_number::PhoneNumber;
mod address;
pub use address::Address;
