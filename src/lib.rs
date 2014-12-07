#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;

pub use faker::Faker;

mod faker;
mod locale;
mod helpers;
pub mod lorem;
pub mod image;


// pub mod name;
// pub mod phone_number;

// pub mod number;