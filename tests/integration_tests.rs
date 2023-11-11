use wemburs::descriptive_statistics; // Use the modules of your crate you want to test

#[test]
fn test_calculate_mean() {
    let data = vec![2.0, 4.0, 6.0];
    let result = descriptive_statistics::mean(&data);
    assert_eq!(result, Ok(4.0));
}
