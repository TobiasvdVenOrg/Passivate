use project_b::multiply_b;

#[test]
fn multiply_2_and_2_is_4_b() {
    let result = multiply_b(2, 2);
    assert_eq!(result, 4);
}