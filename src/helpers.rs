use rand::distributions::range::SampleRange;
use rand::{thread_rng, Rand, Rng};

pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T> {
    let mut newvec = arr.to_vec();
    thread_rng().shuffle(&mut *newvec);
    newvec
}

pub fn replace_sym_with_number(string: String) -> String {
    string
        .chars()
        .map(|x| match x {
            'X' => number_in_range(0i32, 9i32).to_string(),
            'Z' => number_in_range(1i32, 9i32).to_string(),
            'N' => number_in_range(2i32, 9i32).to_string(),
            '#' => number_in_range(0i32, 9i32).to_string(),
            other => other.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn number<T: Rand>() -> T {
    thread_rng().gen()
}

pub fn number_in_range<T: SampleRange + PartialOrd>(min: T, max: T) -> T {
    if (min < max) || (min > max) {
        thread_rng().gen_range(min, max)
    } else {
        min
    }
}

pub fn array_element<'a, T>(array: &'a [T]) -> &T {
    let index = number_in_range(0, array.len() - 1);
    &array[index]
}

pub fn sentence_case(str: String) -> String {
    str.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

pub fn lowercase(str: String) -> String {
    str.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_lowercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        static TEST_ARRAY: [&'static str; 5] = ["One", "Two", "Three", "Four", "Five"];

        let a = shuffle(&TEST_ARRAY);
        // Test that we got back an array of the same length
        assert_eq!(a.len(), 5);
        // Test thhat at least one of the array elements are not in the original position
        // (This test can fail in the event that it "randomly" returns all elements in the original order)
        assert!(
            a[0] != "One" || a[1] != "Two" || a[2] != "Three" || a[3] != "Four" || a[4] != "Five"
        );
    }

    #[test]
    fn test_helper_number() {
        let a: i32 = number();
        let b = 1;
        assert_eq!(a / b, a);
    }

    #[test]
    fn test_helper_number_in_range() {
        for _ in 0..1000 {
            let a = number_in_range(-3, 42);
            assert!(a >= -3 && a <= 42);
            assert_eq!(number_in_range(0, 0), 0);
            assert_eq!(number_in_range(-12, -12), -12);
        }
        for _ in 0..1000 {
            let a = number_in_range(10, 42);
            assert!(a >= 10 && a <= 42);
            assert_eq!(number_in_range(0, 0), 0);
            assert_eq!(number_in_range(3_000_000, 3_000_000), 3_000_000);
        }
    }

    #[test]
    #[should_panic]
    fn test_number_in_range_panic() {
        number_in_range(9, 7);
    }

    #[test]
    fn test_array_element() {
        let a = [1];
        for _ in 0..1000 {
            // test that we dont try to access an index that is out of bounds
            array_element(&a);
        }
    }
}
