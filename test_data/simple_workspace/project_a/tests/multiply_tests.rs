use project_a::multiply_a;

#[test]
fn multiply_2_and_2_is_4_a() {
    let result = multiply_a(2, 2);
    assert_eq!(result, 4);
}