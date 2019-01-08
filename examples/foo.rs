const SOURCE: &'static str = r#"

    222 + 5 + (2 + 3) + foo(2, 233 - bar(666)) + 1

"#;

fn main() {
    let foo = finger::calc(SOURCE.to_owned());
    println!("See this finger? I code rust with it: {}", foo);
}