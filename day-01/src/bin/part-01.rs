fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let mut calibration_numbers: Vec<u32> = Vec::with_capacity(input.len());
    for line in input.lines() {
        let mut nums: Vec<u32> = Vec::new();
        for ch in line.chars() {
            if ch.is_numeric() {
                let num: u32 = match ch.to_digit(10) {
                    Some(n) => n,
                    None => continue,
                };
                nums.push(num);
            }
        }
        let first = match nums.first() {
            Some(n) => n,
            None => continue,
        };
        let last = match nums.last() {
            Some(n) => n,
            None => continue,
        };

        let hidden_num: u32 = format!("{first}{last}").parse::<u32>().unwrap();

        calibration_numbers.push(hidden_num);
    }

    calibration_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let result = part1(test_input);
        assert_eq!(result, 142);
    }
}
