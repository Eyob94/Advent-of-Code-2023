#![allow(unused_variables)]
fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);

    println!("{result}");
}

fn part2(input: &str) -> u32 {
    let parsed_records = input
        .lines()
        .map(|c| {
            c.split(&[':'][..])
                .map(|n| n.trim())
                .map(|n| n.replace(" ", ""))
                .filter(|s| s.chars().nth(0).unwrap_or('.').is_numeric())
                .map(|n| n.parse::<u64>().unwrap_or(0))
                .collect::<Vec<u64>>()
        })
        .flatten()
        .collect::<Vec<u64>>();

    let mut way = 0;

    for time in 0..parsed_records[0] {
        if (time * (parsed_records[0] - time)) > parsed_records[1] {
            way += 1
        }
    }

    way
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        let input = include_str!("./test.txt");
        let result = part2(input);

        assert_eq!(result, 71503)
    }
}
