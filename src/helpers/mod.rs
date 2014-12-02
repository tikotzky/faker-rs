use std::rand::{task_rng, Rng};

pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T>{
    let mut newvec = arr.to_vec();
    task_rng().shuffle(&mut *newvec);
    newvec
}

mod tests;