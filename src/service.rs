/// Basic input. Creates String and put there read lines.
pub fn input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Incorrect value");
    input
}
