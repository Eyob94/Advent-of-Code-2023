#![allow(unused_variables)]
// use std::collections::HashSet;

use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let result = part01(input);

    println!("{result}");
}

fn part01(input: &str) -> u32 {
    let mut points: Vec<u32> = Vec::with_capacity(input.len());
    for line in input.lines() {
        let split_strings = line
            .split(&[':', '|', '\n', '\r'][..])
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let split_numbers = split_strings
            .iter()
            .map(|sc| {
                let new_vec = sc.split(" ").filter(|s| s.len() > 0).collect::<Vec<&str>>();

                new_vec
            })
            .collect::<Vec<Vec<&str>>>();

        let candidate_numbers: HashSet<&str> = HashSet::from_iter(split_numbers[1].iter().cloned());
        let lottery_numbers: HashSet<&str> = HashSet::from_iter(split_numbers[2].iter().cloned());

        let won: usize = candidate_numbers.intersection(&lottery_numbers).count();
        if won > 0 {
            points.push(2_u32.pow((won.saturating_sub(1)).try_into().unwrap_or(0)))
        }
    }

    points.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        let input = include_str!("./test.txt");
        let result = part01(input);

        assert_eq!(result, 13)
    }
}
