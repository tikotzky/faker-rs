use std::rand::{task_rng, Rng, Rand};
use std::rand::distributions::range::SampleRange;
use std::num::NumCast;

pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T>{
    let mut newvec = arr.to_vec();
    task_rng().shuffle(&mut *newvec);
    newvec
}

pub fn replace_sym_with_number(string: String) -> String {
    string.chars().map(|x| match x {
        'X' => number_in_range(0i, 9i).to_string(),
        'Z' => number_in_range(1i, 9i).to_string(),
        'N' => number_in_range(2i, 9i).to_string(),
        other => other.to_string() 
    }).collect::<Vec<String>>().connect("")
}

pub fn number<T: Rand>() -> T {
    task_rng().gen()
}

pub fn number_in_range<T: NumCast + Add<T,T> + SampleRange + PartialOrd>(min: T, max: T) -> T {
    task_rng().gen_range(min, max + NumCast::from(1i).unwrap())
}

pub fn array_element<T>(array: &[T]) -> &T {
    let index = number_in_range(0, array.len() - 1);
    &array[index]
}

#[cfg(test)]
mod tests;