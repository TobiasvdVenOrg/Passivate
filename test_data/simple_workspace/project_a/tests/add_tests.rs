use project_a::add_a;

#[test]
fn add_2_and_2_is_4_a() {
    let result = add_a(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn add_2_and_4_is_6_a() {
    let result = add_a(2, 4);
    assert_eq!(result, 6);
}