use std::rand::{task_rng, Rng};
use super::random;

pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T>{
    let mut newvec = arr.to_vec();
    task_rng().shuffle(&mut *newvec);
    newvec
}

pub fn replace_sym_with_number(string: String) -> String {
	let mut vec = Vec::new();
	for char in string.chars() {
		if char == '#' {
			vec.push(random::number_in_range(0i, 9i).to_string());
		} else {
			vec.push(char.to_string());
		}
	}
	vec.connect("")
}


mod tests;