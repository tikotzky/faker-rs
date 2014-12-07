#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;

pub use faker::Faker;

mod faker;
mod locale;
pub mod lorem;
mod helpers;

// pub mod name;
// pub mod phone_number;
// pub mod image;
// pub mod number;