pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for b in string.bytes() {
        match b {
            b'[' | b'(' | b'{' => stack.push(b),
            b')' if stack.pop() != Some(b'(') => return false,
            b']' | b'}' if stack.pop() != Some(b - 2) => return false,
            _ => {}
        }
    }

    stack.is_empty()
}
