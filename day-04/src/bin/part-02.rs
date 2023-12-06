use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = part02(input);

    println!("{result}");
}

fn part02(input: &str) -> u32 {
    let mut points: HashMap<u32, u32> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let copies = points.entry((i + 1).try_into().unwrap()).or_insert(0);
        *copies += 1;

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

        let matches: usize = candidate_numbers.intersection(&lottery_numbers).count();
        if matches <= 0 {
            continue;
        }

        for _ in 0..*copies {
            for j in 1..matches + 1 {
                let key: u32 = (i + j + 1).try_into().unwrap_or(0);
                let value: &u32 = match points.get(&key) {
                    Some(n) => n,
                    None => &0,
                };
                points.insert(key, value + 1);
            }
        }
    }

    points.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        let input = include_str!("./test.txt");
        let result = part02(input);

        assert_eq!(result, 30)
    }
}
