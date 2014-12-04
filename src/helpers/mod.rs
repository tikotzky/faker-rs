extern crate toml;
use std::io::File;
use std::rand::{task_rng, Rng};
use super::random;

pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T>{
    let mut newvec = arr.to_vec();
    task_rng().shuffle(&mut *newvec);
    newvec
}

pub fn replace_sym_with_number(string: String) -> String {
    string.chars().map(|x| match x {
        'X' => random::number_in_range(0i, 9i).to_string(),
        'Z' => random::number_in_range(1i, 9i).to_string(),
        'N' => random::number_in_range(2i, 9i).to_string(),
        other => other.to_string() 
    }).collect::<Vec<String>>().connect("")
}

pub fn get_value(file_path: &str, toml_path: &str) -> String{
    let path = Path::new(file_path);
    let mut file = File::open(&path).unwrap();
    let toml = file.read_to_string().unwrap();
    let value: toml::Value = from_str(toml.as_slice()).unwrap();
    value.lookup(toml_path).unwrap().to_string()
}



mod tests;