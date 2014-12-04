#[test]
fn test_number(){
    let a = super::number::<int>();
    let b = 1i;
    assert_eq!(a/b, a);
}

#[test]
fn test_number_in_range(){
    for _ in range(0u, 1000) {
        let a = super::number_in_range(-3i, 42);
        assert!(a >= -3 && a <= 42);
        assert_eq!(super::number_in_range(0i, 0), 0);
        assert_eq!(super::number_in_range(-12i, -12), -12);
    }

    for _ in range(0u, 1000) {
        let a = super::number_in_range(10i, 42);
        assert!(a >= 10 && a <= 42);
        assert_eq!(super::number_in_range(0i, 0), 0);
        assert_eq!(super::number_in_range(3_000_000u, 3_000_000), 3_000_000);
    }
}

#[test]
#[should_fail]
fn test_number_in_range_panic(){
    super::number_in_range(9i, 7);
}

#[test]
fn test_array_element(){
    let a : [int, ..1] = [1];
    for _ in range(0u, 1000) {
        // test that we dont try to access an index that is out of bounds
        super::array_element(&a);
    } 
}