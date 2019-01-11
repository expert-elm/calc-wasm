#[test]
fn t_01() {
    let result = finger::calc("1 + 2 * (sin(90) + 3) + 4 / 2".into());
    assert_eq!(result, "11");
}