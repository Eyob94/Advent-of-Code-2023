fn main() {
    let input_data = include_str!("./input.txt");
    let result = part1(input_data);

    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let mut square: Vec<Vec<char>> = Vec::new();
    let mut part_numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        square.push(line.chars().collect());
    }
    for (i, line) in square.iter().enumerate() {
        let mut j: usize = 0;
        while j < line.len() {
            let mut num: String = String::new();
            while line[j].is_numeric() {
                num.push(line[j]);
                j += 1;
                if j >= line.len() {
                    break;
                }
            }
            if num.is_empty() {
                j += 1;
                continue;
            }
            let num_length = num.len();

            let upper_row: &[char] = &square[i.saturating_sub(1)]
                [j.saturating_sub(num_length + 1)..=j.min(line.len() - 1)];
            let lower_row: &[char] = &square[(i + 1).min(square.len() - 1)]
                [j.saturating_sub(num_length + 1)..=j.min(line.len() - 1)];
            let same_row: &[char] = &[
                line[j.saturating_sub(num_length + 1)],
                line[(j).min(line.len() - 1)],
            ];

            let neighbours: Vec<char> = [upper_row, lower_row, same_row]
                .concat()
                .iter()
                .filter(|&c| !c.is_numeric() && *c != '.')
                .map(|c| c.clone())
                .collect();

            if neighbours.len() > 0 {
                part_numbers.push(num.parse::<u32>().unwrap_or(0));
            }
            j += 1;
        }
    }
    part_numbers.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        let test_data = include_str!("./test.txt");
        let test_result = part1(test_data);
        assert_eq!(test_result, 4361);
    }
}
