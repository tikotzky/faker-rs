#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;

pub mod helpers;
pub mod lorem;
pub mod name;
pub mod phone_number;
pub mod image;
pub mod number;
pub mod faker;