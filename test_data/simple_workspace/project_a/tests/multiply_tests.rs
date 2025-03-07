use project_a::multiply;

#[test]
fn multiply_2_and_2_is_4() {
    let result = multiply(2, 2);
    assert_eq!(result, 4);
}