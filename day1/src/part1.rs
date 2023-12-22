mod common;

fn main() {
    let input = common::get_input();
    let result = day1_part1(&input);

    println!("The result is: {}", result);
}

fn day1_part1(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .map(|line| {
            let num1 = line.chars().next().unwrap_or('0');
            let num2 = line.chars().last().unwrap_or('0');
            (num1.to_string() + &num2.to_string())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_case = common::get_lines(
            &r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#
            .to_string(),
        );

        assert_eq!(day1_part1(&test_case), 142);
    }
}
