extern crate regex;
use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn first_digit(input: &str) -> String {
    // Map for converting digit words to their numeric counterparts
    let digit_map = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .copied()
    .collect::<std::collections::HashMap<_, _>>();

    // Regex to match a digit or a digit word
    let digit_re =
        Regex::new(r"(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    // Find the first match
    let first_match = digit_re.find(input).unwrap().as_str();
    // println!("first_digit > first_match: {}", first_match);

    // Convert first match from word to digit if necessary
    let first_digit = digit_map.get(first_match).unwrap_or(&first_match);
    // println!("first_digit: {}", first_digit);
    first_digit.to_string()
}

fn last_digit_(input: &str) -> String {
    let digit_map = [
        ("orez", "0"),
        ("eno", "1"),
        ("owt", "2"),
        ("eerht", "3"),
        ("ruof", "4"),
        ("evif", "5"),
        ("xis", "6"),
        ("neves", "7"),
        ("thgie", "8"),
        ("enin", "9"),
    ]
    .iter()
    .copied()
    .collect::<std::collections::HashMap<_, _>>();

    // Regex to match a digit or a digit word
    let digit_re =
        Regex::new(r"(orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();

    // Find the first match
    let reversed = input.chars().rev().collect::<String>();
    let first_match = digit_re.find(&reversed).unwrap().as_str();
    // println!("last_digit > first_match: {}", first_match);

    // Convert first match from word to digit if necessary
    let last_digit = digit_map.get(first_match).unwrap_or(&first_match);

    // Concatenate the two digits
    last_digit.to_string()
}

fn first_last_digit(input: &str) -> String {
    let first_digit = first_digit(input);
    let last_digit = last_digit_(input);
    format!("{}{}", first_digit, last_digit)
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| first_last_digit(line).parse::<i64>().unwrap())
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string())
    }

    #[test]
    fn it_parses_29() {
        let result = first_last_digit("two1nine");
        assert_eq!(result, "29".to_string());
    }

    #[test]
    fn it_parses_83() {
        let result = first_last_digit("eightwothree");
        assert_eq!(result, "83".to_string());
    }

    #[test]
    fn it_parses_13() {
        let result = first_last_digit("abcone2threexyz");
        assert_eq!(result, "13".to_string());
    }

    #[test]
    fn it_parses_24() {
        let result = first_last_digit("xtwone3four");
        assert_eq!(result, "24".to_string());
    }

    #[test]
    fn it_parses_42() {
        let result = first_last_digit("4nineeightseven2");
        assert_eq!(result, "42".to_string());
    }

    #[test]
    fn it_parses_14() {
        let result = first_last_digit("zoneight234");
        assert_eq!(result, "14".to_string());
    }

    #[test]
    fn it_parses_76() {
        let result = first_last_digit("7pqrstsixteen");
        assert_eq!(result, "76".to_string());
    }

    #[test]
    fn it_parses_one_eight() {
        let result = first_last_digit("oneight");
        assert_eq!(result, "18".to_string());
    }

}
