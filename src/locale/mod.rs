#[deriving(Clone)]
pub struct Locale<'a> {
    pub lorem:              Vec<&'static str>,
    
    pub image_categories:    Vec<&'static str>,

    pub name_first:          Vec<&'static str>,
    pub name_last:           Vec<&'static str>,
    pub name_prefix:         Vec<&'static str>,
    pub name_suffix:         Vec<&'static str>,
    pub name_title:          Vec<&'static str>,

    pub phone_formats:       Vec<&'static str>
}

pub mod en;
mod es;
mod fr;