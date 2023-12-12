extern crate regex;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Debug)]
struct Draw {
    color: Color,
    quantity: i64,
}

fn parse_game(input: &str) -> (i64, i64) {
    let splited = input.split(":").collect::<Vec<&str>>();
    let game_header = splited[0];
    let game_body = splited[1];

    let game_id_re =
        Regex::new(r"Game (?P<game_id>\d+)").unwrap();

    let game_id = game_id_re.captures(game_header).and_then(|cap| {
        cap.name("game_id").map(|m| m.as_str())
    });
    println!("game: {:?}", game_id);

    let cubes_re = Regex::new(r"(?P<quantity>\d+) (?P<color>blue|red|green)").unwrap();
    let draws = cubes_re.captures_iter(game_body).filter_map(|cap| {
        let quantity = cap.name("quantity").map(|m| m.as_str().parse::<i64>().unwrap());
        let color = cap.name("color").map(|m| m.as_str());

        match (color, quantity) {
            (Some(color), Some(quantity)) => Some(Draw {
                color: match color {
                    "blue" => Color::Blue,
                    "red" => Color::Red,
                    "green" => Color::Green,
                    _ => panic!("no matched color"),
                },
                quantity,
            }),
            _ => None,
        }
    }).collect::<Vec<Draw>>();


    let blue_draws: Vec<&Draw> = draws.iter().filter(|d| matches!(d.color, Color::Blue)).collect();
    let red_draws: Vec<&Draw> = draws.iter().filter(|d| matches!(d.color, Color::Red)).collect();
    let green_draws: Vec<&Draw> = draws.iter().filter(|d| matches!(d.color, Color::Green)).collect();

    let max_blue = blue_draws.iter().map(|d| d.quantity).max().unwrap_or(0);
    let max_red = red_draws.iter().map(|d| d.quantity).max().unwrap_or(0);
    let max_green = green_draws.iter().map(|d| d.quantity).max().unwrap_or(0);

    let power = max_blue * max_red * max_green;


    (game_id.unwrap().parse::<i64>().unwrap(), power)
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| parse_game(line))
        .map(|(_, power)| power)
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286".to_string());
    }

    #[test]
    fn it_parses_game() {
        let result = parse_game("Game 3: 3 blue, 4 red; 1 red, 2 green, 2 blue; 2 green");
        assert_eq!(result, (3, 24));
    }
}
