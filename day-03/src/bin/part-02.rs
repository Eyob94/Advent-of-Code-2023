#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
fn main() {
    let input_data = include_str!("./input.txt");
    let result = part2(input_data);

    println!("{result}");
}

fn check_left(left: &str) -> u32 {
    let mut l = String::new();

    for n in left.chars().rev().into_iter().enumerate() {
        if n.0 != 0 && !n.1.is_numeric() {
            break;
        } else if !n.1.is_numeric() {
            continue;
        }

        l.push(n.1);
    }
    let l: String = l.chars().rev().collect();

    l.parse::<u32>().unwrap_or(1)
}
fn check_right(right: &str) -> u32 {
    let mut r = String::new();

    for n in right.chars().into_iter().enumerate() {
        if n.0 != 0 && !n.1.is_numeric() {
            break;
        } else if !n.1.is_numeric() {
            continue;
        }

        r.push(n.1);
    }

    r.parse::<u32>().unwrap_or(1)
}
fn part2(input: &str) -> u32 {
    let mut square: Vec<String> = Vec::new();
    let mut final_answer: Vec<u32> = Vec::new();

    for line in input.lines() {
        square.push(line.to_string());
    }
    for (i, line) in square.iter().enumerate() {
        for (j, ch) in line.chars().into_iter().enumerate() {
            if ch != '*' {
                continue;
            }
            let mut ratios: Vec<u32> = Vec::with_capacity(2);
            if i > 0 {
                let mut upper_row = &square[i - 1];
                let left = upper_row
                    .chars()
                    .clone()
                    .nth(j.saturating_sub(1))
                    .unwrap()
                    .is_numeric();
                let right = upper_row
                    .chars()
                    .clone()
                    .nth((j + 1).min(line.len() - 1))
                    .unwrap_or('.')
                    .is_numeric();
                if left && right != false && upper_row.chars().nth(j).unwrap_or('.').is_numeric() {
                    ratios.push(
                        match upper_row[j.saturating_sub(1)..=(j + 1).min(line.len() - 1)]
                            .parse::<u32>()
                        {
                            Ok(n) => n,
                            Err(_) => panic!("OOOO"),
                        },
                    );
                } else {
                    if left {
                        ratios.push(check_left(&upper_row[..=j]));
                    }
                    if right {
                        ratios.push(check_right(&upper_row[j..]));
                    }
                }
            }
            if i < line.len() - 1 {
                let mut lower_row = &square[i + 1];
                let left = lower_row
                    .chars()
                    .clone()
                    .nth(j.saturating_sub(1))
                    .unwrap()
                    .is_numeric();
                let right = lower_row
                    .chars()
                    .clone()
                    .nth((j + 1).min(line.len() - 1))
                    .unwrap_or('.')
                    .is_numeric();
                if left && right != false && lower_row.chars().nth(j).unwrap_or('.').is_numeric() {
                    ratios.push(
                        match lower_row[j.saturating_sub(1)..=(j + 1).min(line.len() - 1)]
                            .parse::<u32>()
                        {
                            Ok(n) => n,
                            Err(_) => panic!("OOOO"),
                        },
                    );
                } else {
                    if left {
                        ratios.push(check_left(&lower_row[..=j]));
                    }
                    if right {
                        ratios.push(check_right(&lower_row[j..]));
                    }
                }
            }

            {
                let mut same_row = line;
                let left = same_row
                    .chars()
                    .clone()
                    .nth(j.saturating_sub(1))
                    .unwrap()
                    .is_numeric();
                let right = same_row
                    .chars()
                    .clone()
                    .nth((j + 1).min(line.len() - 1))
                    .unwrap_or('.')
                    .is_numeric();
                let mut n = String::new();
                if left {
                    ratios.push(check_left(&same_row[..=j]));
                }
                if right {
                    ratios.push(check_right(&same_row[j..]));
                }
            }

            if ratios.len() < 2 {
                continue;
            }

            let product = ratios.iter().fold(1, |b, n| b * n);

            final_answer.push(product);
        }
    }
    final_answer.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        let test_data = include_str!("./test.txt");
        let test_result = part2(test_data);
        assert_eq!(test_result, 467835);
    }
}
