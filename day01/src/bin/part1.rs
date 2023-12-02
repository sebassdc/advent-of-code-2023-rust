extern crate regex;
use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn first_last_digit(input: &str) -> String {
    let first_digit_re = Regex::new(r"\d").unwrap();
    let first_match = first_digit_re.find(input).unwrap().as_str();
    // println!("First digit: {}", first_match);

    // Regex for all digits
    let all_digits_re = Regex::new(r"\d").unwrap();
    let last_match = all_digits_re.find_iter(input).last().unwrap().as_str();
    // println!("Last digit: {}", last_match);
    first_match.to_string() + last_match
}

fn part1(input: &str) -> String {
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
        let result = part1(
            "1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet",
        );
        assert_eq!(result, "142".to_string())
    }

    #[test]
    fn it_gets_first_and_last_digit() {
        let result = first_last_digit("1kadsfjlkaj2asdfdf");
        assert_eq!(result, "12".to_string());
    }
}
