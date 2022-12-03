use std::fmt::Display;
use std::time::Instant;

use aoc_2022::get_input_as_string;

fn calculate_score(input: &str, version: &str) -> u32 {
    input
        .split("\n")
        .filter(|a| !a.is_empty())
        .map(|x| {
            let ennemy: i32 = (x.chars().nth(0).unwrap() as i32) - 65;
            let mut myself: i32 = (x.chars().nth(2).unwrap() as i32) - 88;

            if "v2".eq(version) {
                myself = (ennemy + myself + 2) % 3;
            }

            let hand_score = myself + 1;
            let fight_score = ((myself - ennemy + 4) % 3) * 3;

            hand_score + fight_score
        })
        .sum::<i32>() as u32
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = calculate_score(input, "v1");
    let p2 = calculate_score(input, "v2");

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::calculate_score;

    #[test]
    fn test_p1() {
        let input = "A Y
B X
C Z";

        let res = calculate_score(&input, "v1");

        assert_eq!(15, res);
    }
}

#[test]
fn test_p2() {
    let input = "A Y
B X
C Z";

    let res = calculate_score(&input, "v2");

    assert_eq!(12, res);
}
