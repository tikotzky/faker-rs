use std::rand::{task_rng, Rng, Rand};
use std::rand::distributions::range::SampleRange;

pub fn number<T: Rand>() -> T {
    task_rng().gen()
}

pub fn number_in_range<T: Add<T,T> + AddOne<T> + SampleRange + PartialOrd>(min: T, max: T) -> T {
    task_rng().gen_range(min, max.add_one())
}

pub fn array_element<T>(array: &[T]) -> &T {
    let index = number_in_range(0, array.len());
    &array[index]
}

pub trait AddOne<T> {
    fn add_one(&self) -> T;
}


impl AddOne<int> for int {
    fn add_one(&self) -> int {
        *self + 1i
    }
}

impl AddOne<uint> for uint {
    fn add_one(&self) -> uint {
        *self + 1u
    }
}

impl AddOne<u8> for u8 {
    fn add_one(&self) -> u8 {
        *self + 1u8
    }
}

impl AddOne<u16> for u16 {
    fn add_one(&self) -> u16 {
        *self + 1u16
    }
}

impl AddOne<u32> for u32 {
    fn add_one(&self) -> u32 {
        *self + 1u32
    }
}

impl AddOne<u64> for u64 {
    fn add_one(&self) -> u64 {
        *self + 1u64
    }
}

impl AddOne<i8> for i8 {
    fn add_one(&self) -> i8 {
        *self + 1i8
    }
}

impl AddOne<i16> for i16 {
    fn add_one(&self) -> i16 {
        *self + 1i16
    }
}

impl AddOne<i32> for i32 {
    fn add_one(&self) -> i32 {
        *self + 1i32
    }
}

impl AddOne<i64> for i64 {
    fn add_one(&self) -> i64 {
        *self + 1i64
    }
}

impl AddOne<f32> for f32 {
    fn add_one(&self) -> f32 {
        *self + 1f32
    }
}

impl AddOne<f64> for f64 {
    fn add_one(&self) -> f64 {
        *self + 1f64
    }
}

mod tests;