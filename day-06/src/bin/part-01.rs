#![allow(unused_variables)]
fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);

    println!("{result}");
}

#[derive(Debug)]
struct Record {
    time: u32,
    distance: u32,
}

fn part1(input: &str) -> u32 {
    let mut records: Vec<Record> = Vec::new();

    let parsed_records = input
        .lines()
        .map(|c| {
            c.split(&[' ', ':'][..])
                .filter(|s| !s.is_empty() && s.chars().nth(0).unwrap_or('.').is_numeric())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    for (i, rec) in parsed_records[0].iter().enumerate() {
        records.push(Record {
            time: rec.parse::<u32>().unwrap_or(0),
            distance: parsed_records[1][i].parse::<u32>().unwrap_or(0),
        });
    }

    let mut ways: Vec<u32> = vec![];

    for record in records.iter() {
        let mut way = 0;
        for time in 0..record.time {
            if (time * (record.time - time)) as u32 > record.distance {
                way += 1
            }
        }
        ways.push(way)
    }

    ways.iter().fold(1, |acc, e| acc * e)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        let input = include_str!("./test.txt");
        let result = part1(input);

        assert_eq!(result, 288)
    }
}
