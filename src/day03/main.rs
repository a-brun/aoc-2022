use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_2022::get_input_as_string;

fn calculate_priority(item: &char) -> u32 {
    if item.is_lowercase() {
        (*item as u32) - 96
    } else {
        (*item as u32) - 38
    }
}

fn get_items_priority(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|x| {
            let (left, right) = x.split_at(x.len() / 2);

            let left_items: HashSet<char> = left.chars().collect();
            let right_items: HashSet<char> = right.chars().collect();

            let intersect: char = *left_items.intersection(&right_items).into_iter().next().unwrap();
            calculate_priority(&intersect)
        })
        .sum()
}

fn get_badges_priority(input: &str) -> u32 {
    let mut sum: u32 = 0;

    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .collect::<Vec<&str>>()
        .chunks(3)
        .for_each(|x| {
            let first: HashSet<char> = x[0].chars().collect();
            let second: HashSet<char> = x[1].chars().collect();
            let third: HashSet<char> = x[2].chars().collect();

            let first_and_second = first.intersection(&second).map(|c| *c).collect::<HashSet<char>>();
            let intersect: char = *first_and_second.intersection(&third).into_iter().next().unwrap();

            sum += calculate_priority(&intersect);
        });

    sum
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_items_priority(input);
    let p2 = get_badges_priority(input);

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
    use crate::get_badges_priority;
    use crate::get_items_priority;

    #[test]
    fn test_p1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let res = get_items_priority(&input);

        assert_eq!(157, res);
    }

    #[test]
    fn test_p2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let res = get_badges_priority(&input);

        assert_eq!(70, res);
    }
}
