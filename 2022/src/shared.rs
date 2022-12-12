pub fn lines_to_owned(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}
