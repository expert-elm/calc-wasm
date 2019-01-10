const SOURCE: &'static str = r#"

    1 + 2 * (sin(0)) + 3 + 4 / 2

"#;

fn main() {
    let foo = finger::calc(SOURCE.to_owned());
    println!("See this finger? I code rust with it: {}", foo);
}