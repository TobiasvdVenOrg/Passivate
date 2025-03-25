use sample_project::repeat;

#[test]
fn repeat_bla_four_times() {
    let result = repeat("bla", 4);
    assert_eq!(result, "blablablabla");
}