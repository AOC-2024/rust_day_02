use day_02::find_safe_reports;

#[test]
fn it_should_find_safe_reports() {
    assert_eq!(find_safe_reports("tests/resources/puzzle.txt"), 2);
}