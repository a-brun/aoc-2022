use std::fmt::Display;
use std::time::Instant;

use aoc_2022::get_input_as_string;

fn count_complete_overlap(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|l| {
            let halves: Vec<&str> = l.split(",").collect();

            let elf1: Vec<u32> = halves[0]
                .split("-")
                .map(|ß| ß.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let elf2: Vec<u32> = halves[1]
                .split("-")
                .map(|ß| ß.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if elf1[0] <= elf2[0] && elf1[1] >= elf2[1] || elf2[0] <= elf1[0] && elf2[1] >= elf1[1] {
                1
            } else {
                0
            }
        })
        .sum()
}

fn count_overlap(input: &str) -> u32 {
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|l| {
            let halves: Vec<&str> = l.split(",").collect();

            let elf1: Vec<u32> = halves[0]
                .split("-")
                .map(|ß| ß.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let elf2: Vec<u32> = halves[1]
                .split("-")
                .map(|ß| ß.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if elf1[0] <= elf2[0] && elf1[1] >= elf2[0] || elf2[0] <= elf1[0] && elf2[1] >= elf1[0] {
                1
            } else {
                0
            }
        })
        .sum()
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = count_complete_overlap(input);
    let p2 = count_overlap(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::count_complete_overlap;
    use crate::count_overlap;

    #[test]
    fn test_p1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let res = count_complete_overlap(&input);

        assert_eq!(2, res);
    }

    #[test]
    fn test_p2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let res = count_overlap(&input);

        assert_eq!(4, res);
    }
}
