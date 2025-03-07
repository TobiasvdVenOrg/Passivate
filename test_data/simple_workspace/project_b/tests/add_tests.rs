use project_b::add_b;

#[test]
fn add_2_and_2_is_4_b() {
    let result = add_b(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn add_2_and_4_is_6_b() {
    let result = add_b(2, 4);
    assert_eq!(result, 6);
}