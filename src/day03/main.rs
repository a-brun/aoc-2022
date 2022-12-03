use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_2022::get_input_as_string;

fn get_total_priority(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|x| {
            let (left, right) = x.split_at(x.len() / 2);

            let left_items: HashSet<char> = left.chars().collect();
            let right_items: HashSet<char> = right.chars().collect();

            let intersect: char = *left_items.intersection(&right_items).into_iter().next().unwrap();

            if intersect.is_lowercase() {
                (intersect as u32) - 96
            } else {
                (intersect as u32) - 38
            }
        })
        .sum()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_total_priority(input);
    let p2 = 9;

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::get_total_priority;

    #[test]
    fn test_p1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let res = get_total_priority(&input);

        assert_eq!(157, res);
    }
}
