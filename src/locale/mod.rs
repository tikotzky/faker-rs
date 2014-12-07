#[deriving(Clone)]
pub struct Locale<'a> {
    pub lorem:              Vec<&'static str>,
    
    pub imageCategories:    Vec<&'static str>,

    pub nameFirst:          Vec<&'static str>,
    pub nameLast:           Vec<&'static str>,
    pub namePrefix:         Vec<&'static str>,
    pub nameSuffix:         Vec<&'static str>,
    pub nameTitle:          Vec<&'static str>
}

pub mod en;
mod es;
mod fr;