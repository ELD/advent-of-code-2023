pub fn part1() -> u32 {
    let parsed_digits = parse_input("day1/input.txt")
        .iter()
        .map(|s| {
            s.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    sum_digits(&parsed_digits)
}

pub fn part2() -> u32 {
    let parsed_digits = process_lines(&parse_input("day1/input.txt"));
    parsed_digits
        .iter()
        .map(|digits| digits.first().expect("no first") * 10 + digits.last().expect("no last"))
        .sum()
}

fn digits_to_number(digits: &[u32]) -> u32 {
    digits.last().expect("no first") + 10 * digits.first().expect("no first")
}

fn sum_digits(list: &[Vec<u32>]) -> u32 {
    list.iter()
        .map(|digits| digits_to_number(digits))
        .sum::<u32>()
}

fn process_lines(input: &[String]) -> Vec<Vec<u32>> {
    input
        .iter()
        .map(|line| {
            (0..line.len())
                .filter_map(|i| {
                    let haystack = &line[i..];
                    let result = if haystack.starts_with("one") {
                        '1'
                    } else if haystack.starts_with("two") {
                        '2'
                    } else if haystack.starts_with("three") {
                        '3'
                    } else if haystack.starts_with("four") {
                        '4'
                    } else if haystack.starts_with("five") {
                        '5'
                    } else if haystack.starts_with("six") {
                        '6'
                    } else if haystack.starts_with("seven") {
                        '7'
                    } else if haystack.starts_with("eight") {
                        '8'
                    } else if haystack.starts_with("nine") {
                        '9'
                    } else {
                        haystack.chars().next().unwrap()
                    };
                    result.to_digit(10)
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>()
}

fn parse_input(path: &str) -> Vec<String> {
    let input = std::fs::read_to_string(path).expect("failed to read path");
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<String>>()
}
