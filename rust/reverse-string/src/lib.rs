pub fn reverse(input: &str) -> String {
    let mut op = String::new();
    for i in input.chars().rev() {
        op.push(i);
    }
    op
}
