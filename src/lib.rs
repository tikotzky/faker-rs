//! Create fake data, which resembles real world data.
//!
//! Fake data is very useful for many reasons, including but not limited to:
//! tests and examples. While fake data looks like real data it is simply
//! **formatted random data**, which means that in many cases the generated
//! values **are in fact real**, beware.
//
// TODO: Avoid real data?

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
