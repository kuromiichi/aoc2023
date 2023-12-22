pub fn get_input() -> Vec<String> {
    get_lines(&include_str!("input.txt").to_string())
}

pub fn get_lines(input: &String) -> Vec<String> {
    input.lines().map(|s| s.trim().to_string()).collect()
}
