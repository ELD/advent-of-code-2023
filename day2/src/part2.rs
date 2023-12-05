use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map,
    IResult,
};
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq)]
struct Round {
    cubes: Vec<Cube>,
}

#[derive(Clone, Debug, PartialEq)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}
impl From<(u32, &str)> for Cube {
    fn from((quantity, color): (u32, &str)) -> Self {
        match color {
            "red" => Cube::Red(quantity),
            "green" => Cube::Green(quantity),
            "blue" => Cube::Blue(quantity),
            _ => panic!("invalid color option"),
        }
    }
}

pub fn solution() -> u32 {
    let games = parse_input("day2/input.txt");
    calculate_power_sets(&games)
}

fn calculate_power_sets(games: &[Game]) -> u32 {
    games.iter().map(|game| {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        game.rounds.iter().for_each(|round| {
            round.cubes.iter().for_each(|cube| {
                match cube {
                    Cube::Red(quantity) if quantity > &max_red => max_red = *quantity,
                    Cube::Green(quantity) if quantity > &max_green => max_green = *quantity,
                    Cube::Blue(quantity) if quantity > &max_blue => max_blue = *quantity,
                    _ => {},
                };
            })
        });
        max_red * max_green * max_blue
    }).sum()
}

fn parse_input(path: &str) -> Vec<Game> {
    let input = read_to_string(path).expect("failed to read file");
    parse_games(&input)
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| parse_game(l))
        .collect::<Vec<IResult<&str, Game>>>()
        .into_iter()
        .filter_map(Result::ok)
        .map(|g| g.1)
        .collect::<Vec<Game>>()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u32>()
    })(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = parse_rounds(input)?;

    Ok((
        input,
        Game {
            id: id.expect("invalid integer"),
            rounds,
        },
    ))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    let (input, rounds) = map(take_while1(|c: char| c != '\n'), |s: &str| {
        s.split("; ").collect::<Vec<&str>>()
    })(input)?;

    let rounds = rounds
        .iter()
        .map(|r| parse_round(r))
        .collect::<Vec<IResult<&str, Round>>>()
        .into_iter()
        .filter_map(Result::ok)
        .map(|r| r.1)
        .collect::<Vec<Round>>();

    Ok((input, rounds))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = map(take_while1(|c: char| c != '\n'), |s: &str| {
        s.split(", ").collect::<Vec<&str>>()
    })(input)?;

    let parsed_cubes = cubes
        .iter()
        .map(|c| parse_cube(c))
        .collect::<Vec<IResult<&str, Cube>>>()
        .into_iter()
        .filter_map(Result::ok)
        .map(|c| c.1)
        .collect::<Vec<Cube>>();

    Ok((
        input,
        Round {
            cubes: parsed_cubes,
        },
    ))
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, quantity) = map(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u32>().expect("invalid integer")
    })(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, cube) = map(alt((tag("red"), tag("green"), tag("blue"))), |s: &str| {
        Cube::from((quantity, s))
    })(input)?;

    Ok((input, cube))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cube() {
        let input = &["3 red", "4 green", "5 blue"];
        let expected = vec![Cube::Red(3), Cube::Green(4), Cube::Blue(5)];

        input.iter().zip(expected).for_each(|(i, e)| {
            assert_eq!(parse_cube(i).expect("error in parsing"), ("", e));
        });
    }

    #[test]
    fn test_parse_round() {
        let input = &["3 blue, 4 red", "1 red, 2 green", "6 blue, 2 green"];
        let expected = vec![
            Round {
                cubes: vec![Cube::Blue(3), Cube::Red(4)],
            },
            Round {
                cubes: vec![Cube::Red(1), Cube::Green(2)],
            },
            Round {
                cubes: vec![Cube::Blue(6), Cube::Green(2)],
            },
        ];

        input.iter().zip(expected).for_each(|(i, e)| {
            assert_eq!(parse_round(i).expect("error parsing round"), ("", e));
        });
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = Game {
            id: 2,
            rounds: vec![
                Round {
                    cubes: vec![Cube::Blue(1), Cube::Green(2)],
                },
                Round {
                    cubes: vec![Cube::Green(3), Cube::Blue(4), Cube::Red(1)],
                },
                Round {
                    cubes: vec![Cube::Green(1), Cube::Blue(1)],
                },
            ],
        };

        assert_eq!(
            parse_game(input).expect("error parsing game"),
            ("", expected)
        );
    }

    const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_example() {
        assert_eq!(calculate_power_sets(&parse_games(TEST_INPUT)), 2286);
    }
}

