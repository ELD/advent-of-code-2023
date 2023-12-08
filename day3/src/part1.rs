use std::{
    collections::HashSet,
    fs::read_to_string,
    iter::{Enumerate, Peekable},
};

type Grid = Vec<Vec<GridCell>>;

#[derive(Debug, Eq, PartialEq)]
struct GridCell {
    x: usize,
    y: usize,
    cell_type: CellType,
}

#[derive(Hash, Debug, Eq, PartialEq)]
enum CellType {
    Period,
    Symbol,
    Number {
        value: u32,
        pairs: Vec<(usize, usize)>,
    },
}

pub fn solution() -> u32 {
    let input = read_to_string("day3/input.txt").unwrap();
    sum_part_numbers(&parse_input(&input))
}

fn sum_part_numbers(grid: &Grid) -> u32 {
    let mut part_numbers = HashSet::new();
    let symbols = grid
        .iter()
        .flatten()
        .filter(|cell| matches!(cell.cell_type, CellType::Symbol))
        .collect::<Vec<_>>();
    let coords_to_check = calculate_adjacencies(grid, &symbols);

    coords_to_check.iter().for_each(|&(x, y)| {
        if let ref cell @ CellType::Number { .. } = grid[y][x].cell_type {
            part_numbers.insert(cell);
        }
    });

    part_numbers
        .iter()
        .filter(|cell| matches!(cell, CellType::Number { .. }))
        .map(|cell| match cell {
            CellType::Number { value, .. } => value,
            _ => unreachable!(),
        })
        .sum()
}

fn calculate_adjacencies(grid: &Grid, symbols: &[&GridCell]) -> HashSet<(usize, usize)> {
    let bound_y = grid.len();
    let bound_x = grid[0].len();
    symbols
        .iter()
        .flat_map(|symbol| {
            let mut adjacencies = vec![];
            let x = symbol.x;
            let y = symbol.y;

            // up-left, up-right, down-left, down-right, up, down, left, right
            adjacencies.push((x - 1, y + 1));
            adjacencies.push((x + 1, y + 1));
            adjacencies.push((x - 1, y - 1));
            adjacencies.push((x + 1, y - 1));
            adjacencies.push((x, y + 1));
            adjacencies.push((x, y - 1));
            adjacencies.push((x - 1, y));
            adjacencies.push((x + 1, y));

            adjacencies
                .iter()
                .filter(|(x, y)| *x < bound_x && *y < bound_y)
                .cloned()
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<(usize, usize)>>()
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| parse_line(y, line))
        .collect()
}

fn parse_line(y: usize, input: &str) -> Vec<GridCell> {
    let mut line_iter = input.chars().enumerate().peekable();
    let mut line = vec![];
    while let Some((x, c)) = line_iter.next() {
        match c {
            '.' => line.push(GridCell {
                x,
                y,
                cell_type: CellType::Period,
            }),
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '+' | '/' | '<'
            | '>' | '?' | '_' | '=' => line.push(GridCell {
                x,
                y,
                cell_type: CellType::Symbol,
            }),
            c if c.is_ascii_digit() => {
                let (number, span) = take_while_digit(c, &mut line_iter);
                let pairs = (0..span).map(|inc| (x + inc, y)).collect::<Vec<_>>();
                (0..span).for_each(|inc| {
                    line.push(GridCell {
                        x: x + inc,
                        y,
                        cell_type: CellType::Number {
                            value: number,
                            pairs: pairs.clone(),
                        },
                    });
                });
            }
            invalid => unreachable!("invalid character {invalid}"),
        }
    }

    line
}

fn take_while_digit(
    first: char,
    iter: &mut Peekable<Enumerate<std::str::Chars<'_>>>,
) -> (u32, usize) {
    let mut number = vec![first];
    while let Some((_, c)) = iter.peek() {
        match c {
            c if c.is_ascii_digit() => {
                number.push(*c);
                iter.next();
            }
            _ => {
                break;
            }
        };
    }
    (
        number.iter().collect::<String>().parse::<u32>().unwrap(),
        number.len(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example_solution() {
        assert_eq!(sum_part_numbers(&parse_input(TEST_INPUT)), 4361);
    }

    #[test]
    fn test_parse_line() {
        let input = "...*..51.#";
        let expected = vec![
            GridCell {
                x: 0,
                y: 0,
                cell_type: CellType::Period,
            },
            GridCell {
                x: 1,
                y: 0,
                cell_type: CellType::Period,
            },
            GridCell {
                x: 2,
                y: 0,
                cell_type: CellType::Period,
            },
            GridCell {
                x: 3,
                y: 0,
                cell_type: CellType::Symbol,
            },
            GridCell {
                x: 4,
                y: 0,
                cell_type: CellType::Period,
            },
            GridCell {
                x: 5,
                y: 0,
                cell_type: CellType::Period,
            },
            GridCell {
                x: 6,
                y: 0,
                cell_type: CellType::Number {
                    value: 51,
                    pairs: vec![(6, 0), (7, 0)],
                },
            },
            GridCell {
                x: 7,
                y: 0,
                cell_type: CellType::Number {
                    value: 51,
                    pairs: vec![(6, 0), (7, 0)],
                },
            },
            GridCell {
                x: 8,
                y: 0,
                cell_type: CellType::Period,
            },
            GridCell {
                x: 9,
                y: 0,
                cell_type: CellType::Symbol,
            },
        ];

        parse_line(0, input)
            .iter()
            .zip(expected)
            .for_each(|(actual, expected)| {
                assert_eq!(actual, &expected);
            });
    }
}
