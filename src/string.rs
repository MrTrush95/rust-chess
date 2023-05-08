pub fn pad_right(s: &str, n: usize, c: char) -> String {
    if s.len() >= n {
        return String::from(s);
    }

    let mut result = String::from(s);
    for _ in 0..n - s.len() {
        result.push(c);
    }
    result
}
