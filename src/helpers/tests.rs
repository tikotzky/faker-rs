#[test]
fn test_shuffle(){
    static TEST_ARRAY: [&'static str, ..5] = [
        "One",
        "Two",
        "Three",
        "Four",
        "Five"
    ];

    let a = super::shuffle(&TEST_ARRAY);
    // Test that we got back an array of the same length
    assert_eq!(a.len(), 5);
    // Test thhat at least one of the array elements are not in the original position
    // (This test can fail in the event that it "randomly" returns all elements in the original order)
    assert!(a[0] != "One" || a[1] != "Two" || a[2] != "Three" || a[3] != "Four" || a[4] != "Five");
}