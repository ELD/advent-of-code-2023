pub fn solution() -> usize {
    let input = std::fs::read_to_string("day6/input.txt").expect("unable to read file");
    calculate_product_of_times(&parse_input(&input))
}

pub fn calculate_product_of_times(&(time, distance): &(usize, usize)) -> usize {
    (0..=time).filter(|&step| {
        let speed = step;
        let remaining_time = time - step;
        let traveled_dist = speed * remaining_time;

        traveled_dist > distance
    }).count()
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let times = lines.next().expect("malformed input");
    let distances = lines.next().expect("malformed input");

    let times = times
        .split(':')
        .skip(1)
        .next()
        .expect("malformed input")
        .split_whitespace()
        .map(|s| s.trim())
        .collect::<String>()
        .parse::<usize>()
        .expect("not a number");
    let distances = distances
        .split(':')
        .skip(1)
        .next()
        .expect("malformed input")
        .split_whitespace()
        .map(|s| s.trim())
        .collect::<String>()
        .parse::<usize>()
        .expect("not a number");

    (times, distances)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_example() {
        let input = parse_input(INPUT);
        dbg!(input);
        assert_eq!(71503, calculate_product_of_times(&input));
    }
}
