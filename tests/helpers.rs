extern crate faker;
use faker::Faker;

#[test]
fn test_shuffle(){
    let faker = Faker::new("en");
    static TEST_ARRAY: [&'static str; 5] = [
        "One",
        "Two",
        "Three",
        "Four",
        "Five"
    ];

    let a = faker.helpers.shuffle(&TEST_ARRAY);
    // Test that we got back an array of the same length
    assert_eq!(a.len(), 5);
    // Test thhat at least one of the array elements are not in the original position
    // (This test can fail in the event that it "randomly" returns all elements in the original order)
    assert!(a[0] != "One" || a[1] != "Two" || a[2] != "Three" || a[3] != "Four" || a[4] != "Five");
}

#[test]
fn test_helper_number(){
    let faker = Faker::new("en");
    let a: i32 = faker.helpers.number();
    let b = 1;
    assert_eq!(a/b, a);
}

#[test]
fn test_helper_number_in_range(){
    let faker = Faker::new("en");
    for _ in 0..1000 {
        let a = faker.helpers.number_in_range(-3, 42);
        assert!(a >= -3 && a <= 42);
        assert_eq!(faker.helpers.number_in_range(0, 0), 0);
        assert_eq!(faker.helpers.number_in_range(-12, -12), -12);
    }
    for _ in 0..1000 {
        let a = faker.helpers.number_in_range(10, 42);
        assert!(a >= 10 && a <= 42);
        assert_eq!(faker.helpers.number_in_range(0, 0), 0);
        assert_eq!(faker.helpers.number_in_range(3_000_000, 3_000_000), 3_000_000);
    }
}

#[test]
#[should_panic]
fn test_number_in_range_panic(){
    let faker = Faker::new("en");
    faker.helpers.number_in_range(9, 7);
}

#[test]
fn test_array_element(){
    let faker = Faker::new("en");
    let a = [1];
    for _ in 0..1000 {
        // test that we dont try to access an index that is out of bounds
        faker.helpers.array_element(&a);
    }
}
