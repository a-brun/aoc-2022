use std::fmt::Display;
use std::time::Instant;

use aoc_2022::get_input_as_string;

fn get_marker_position(input: &str, length: usize) -> usize {
    let elements = input.chars().collect::<Vec<char>>();
    let mut count = 0;

    for i in 0..elements.len() {
        if elements[i + 1..i + length - count].iter().all(|c| c != &elements[i]) {
            count += 1;
        } else {
            count = 0;
        }

        if count == length {
            return i + 1;
        }
    }

    0
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_marker_position(&input, 4);
    let p2 = get_marker_position(&input, 14);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day06.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::get_marker_position;

    #[test]
    fn test_p1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        let res = get_marker_position(&input, 4);

        assert_eq!(7, res);
    }

    #[test]
    fn test_p2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        let res = get_marker_position(&input, 14);

        assert_eq!(19, res);
    }
}
