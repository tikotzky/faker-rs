use std::rand::{task_rng, Rng, Rand};
use std::rand::distributions::range::SampleRange;
use std::num::NumCast;

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

mod tests;