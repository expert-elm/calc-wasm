fn main() {
    let foo = finger::calc("1 + avg(2, 4, 6) * sin(90) + rand(10, 100) / (-0.2 / (-2) * 100)".into());
    println!("See this finger? I code rust with it: {}", foo);
}
