#[deriving(Clone)]
pub struct Locale<'a> {
    pub lorem:              Vec<&'static str>,
    pub imageCategories:    Vec<&'static str>
}

pub mod en;
mod es;
mod fr;