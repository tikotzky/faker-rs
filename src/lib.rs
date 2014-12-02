#![feature(phase)]
extern crate regex;
#[phase(plugin)] 
extern crate regex_macros;

pub mod helpers;
pub mod lorem;
pub mod name;
pub mod random;
