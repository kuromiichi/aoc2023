mod common;

fn main() {
    let input = common::get_input();
    let result = day1_part2(&input);

    println!("The result is: {}", result);
}

fn get_numbers_from_line(line: &String) -> String {
    let mut line = line.clone();

    let numbers = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    numbers.iter().for_each(|(key, value)| {
        line = line.replace(key, &value);
    });

    line.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn day1_part2(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| get_numbers_from_line(&line))
        .map(|line| {
            let num1 = line.chars().next().unwrap_or('0');
            let num2 = line.chars().last().unwrap_or('0');
            format!("{}{}", num1, num2).parse::<i32>().unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Returns a string of digits when given a string containing words representing numbers
    #[test]
    fn test_returns_string_of_digits_when_given_string_with_words_representing_numbers() {
        let line = String::from("one two three");
        let result = get_numbers_from_line(&line);
        assert_eq!(result, "123");
    }

    // Handles input with no words representing numbers and returns an empty string
    #[test]
    fn test_handles_input_with_no_words_representing_numbers() {
        let line = String::from("abc def ghi");
        let result = get_numbers_from_line(&line);
        assert_eq!(result, "");
    }

    // Handles input with multiple occurrences of the same word representing a number
    #[test]
    fn test_handles_input_with_multiple_occurrences_of_the_same_word_representing_a_number() {
        let line = String::from("one one one");
        let result = get_numbers_from_line(&line);
        assert_eq!(result, "111");
    }

    #[test]
    fn test_part2() {
        let test_case = common::get_lines(
            &r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        "#
            .to_string(),
        );

        let result = day1_part2(&test_case);
        assert_eq!(result, 281);
    }
}
