use std::rand::{task_rng, Rng, Rand};
use std::rand::distributions::range::SampleRange;
use std::num::NumCast;

pub struct Helpers;

impl Helpers {

    pub fn new() -> Helpers{
        Helpers
    }

    pub fn shuffle<T: Clone>(&self, arr: &[T]) -> Vec<T>{
        let mut newvec = arr.to_vec();
        task_rng().shuffle(&mut *newvec);
        newvec
    }    

    pub fn replace_sym_with_number(&self, string: String) -> String {
        string.chars().map(|x| match x {
            'X' => self.number_in_range(0i, 9i).to_string(),
            'Z' => self.number_in_range(1i, 9i).to_string(),
            'N' => self.number_in_range(2i, 9i).to_string(),
            '#' => self.number_in_range(0i, 9i).to_string(),
            other => other.to_string() 
        }).collect::<Vec<String>>().connect("")
    }

    pub fn number<T: Rand>(&self) -> T {
        task_rng().gen()
    }

    pub fn number_in_range<T: NumCast + Add<T,T> + SampleRange + PartialOrd>(&self, min: T, max: T) -> T {
        task_rng().gen_range(min, max + NumCast::from(1i).unwrap())
    }

    pub fn array_element<'a, T>(&'a self, array: &'a [T]) -> &T {
        let index = self.number_in_range(0, array.len() - 1);
        &array[index]
    }

}

#[cfg(test)]
mod tests;