use std::rand::{task_rng, Rng, Rand};
use std::rand::distributions::range::SampleRange;

pub fn number<T: Rand>() -> T {
    task_rng().gen()
}

pub fn number_in_range<T: SampleRange + PartialOrd>(min: T, max: T) -> T {
    task_rng().gen_range(min, max)
}

pub fn array_element<T>(array: &[T]) -> &T {
    let index = number_in_range(0, array.len());
    &array[index]
}