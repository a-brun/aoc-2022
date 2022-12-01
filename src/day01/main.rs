use std::fmt::Display;
use std::time::Instant;

use itertools::Itertools;

use aoc_2022::get_input_as_string;

fn get_calories_of_biggest_snacks(input: &str, n: usize) -> u32 {
    input
        .split("\n\n")
        .map(|s| s.split("\n").map(|c| c.parse::<u32>().unwrap_or(0)).sum::<u32>())
        .sorted()
        .rev()
        .take(n)
        .sum()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_calories_of_biggest_snacks(input, 1);
    let p2 = get_calories_of_biggest_snacks(input, 3);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::get_calories_of_biggest_snacks;

    #[test]
    fn test_p1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let res = get_calories_of_biggest_snacks(&input, 1);

        assert_eq!(24000, res);
    }

    #[test]
    fn test_p2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let res = get_calories_of_biggest_snacks(&input, 3);

        assert_eq!(45000, res);
    }
}
