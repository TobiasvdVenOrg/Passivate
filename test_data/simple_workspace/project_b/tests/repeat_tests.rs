use project_b::repeat_b;

#[test]
fn repeat_bla_four_times() {
    let result = repeat_b("bla", 4);
    assert_eq!(result, "blablablabla");
}