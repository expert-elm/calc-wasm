#[test]
fn add() {
    let result = finger::calc("1 + 1".into());
    assert_eq!(result, "2");
}

#[test]
fn sub() {
    let result = finger::calc("234 - 1".into());
    assert_eq!(result, "233");
}

#[test]
fn mul() {
    let result = finger::calc("128 * 2".into());
    assert_eq!(result, "256");
}

#[test]
fn div() {
    let result = finger::calc("1024 / 2".into());
    assert_eq!(result, "512");
}

#[test]
fn sin() {
    let result = finger::calc("sin(90)".into());
    assert_eq!(result, "1");
}

#[test]
fn cos() {
    let result = finger::calc("cos(0)".into());
    assert_eq!(result, "1");
}

#[test]
fn add_mul() {
    let result = finger::calc("1 + 1 * 2".into());
    assert_eq!(result, "3");
}

#[test]
fn add_div() {
    let result = finger::calc("2 + 4 / 2".into());
    assert_eq!(result, "4");
}

#[test]
fn random() {
    let a = finger::calc("rand()".into());
    let b = finger::calc("rand()".into());
    assert_ne!(a, b);
}

#[test]
fn decimal() {
    let a = finger::calc("0.2 / 2".into());
    assert_eq!(a, "0.1");
}

#[test]
fn negative() {
    let a = finger::calc("-0.2 / 2".into());
    assert_eq!(a, "-0.1");
}