#![feature(phase)]
extern crate regex;
#[phase(plugin)]
extern crate regex_macros;
extern crate faker;
pub use faker::Faker;

pub mod helpers;
pub mod image;
pub mod lorem;
pub mod name;
pub mod phone_number;
pub mod number;