use sample_project::add;

#[test]
fn add_2_and_2_is_4() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn add_2_and_4_is_6() {
    let result = add(2, 4);
    assert_eq!(result, 6);
}