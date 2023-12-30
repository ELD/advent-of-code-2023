pub fn solution() -> usize {
    let input = std::fs::read_to_string("day6/input.txt").expect("unable to read file");
    calculate_product_of_times(&parse_input(&input))
}

pub fn calculate_product_of_times(input: &[(usize, usize)]) -> usize {
    input.iter().map(|&(t, d)| {
        // Step from the starting time to the ending time, forward project the distance. Use two
        // pointers from the end and beginning. When both pointers are valid solutions, take the
        // range as a count
        let count = (0..=t).filter(|&step| {
            let speed = step;
            let remaining_time = t - step;
            let distance = speed * remaining_time;

            distance > d
        }).count();
        count
    }).product()
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();
    let times = lines.next().expect("malformed input");
    let distances = lines.next().expect("malformed input");

    let times = times
        .split(':')
        .skip(1)
        .next()
        .expect("malformed input")
        .split_whitespace()
        .map(|s| s.trim().parse().expect("not a number"))
        .collect::<Vec<usize>>();
    let distances = distances
        .split(':')
        .skip(1)
        .next()
        .expect("malformed input")
        .split_whitespace()
        .map(|s| s.trim().parse().expect("not a number"))
        .collect::<Vec<usize>>();

    times.into_iter().zip(distances).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_example() {
        let input = parse_input(INPUT);
        assert_eq!(288, calculate_product_of_times(&input));
    }
}
