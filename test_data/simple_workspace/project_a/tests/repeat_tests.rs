use project_a::repeat_a;

#[test]
fn repeat_bla_four_times() {
    let result = repeat_a("bla", 4);
    assert_eq!(result, "blablablabla");
}