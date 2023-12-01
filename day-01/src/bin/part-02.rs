fn main() {
    let input = include_str!("./input2.txt");
    let result = part2(input);
    println!("{result}");
}

struct StrToNum {
    str: &'static str,
    num: u32,
}

fn part2(input: &str) -> u32 {
    let three_letter_digits: [StrToNum; 3] = [
        StrToNum { str: "one", num: 1 },
        StrToNum { str: "two", num: 2 },
        StrToNum { str: "six", num: 6 },
    ];
    let four_letter_digits: [StrToNum; 3] = [
        StrToNum {
            str: "four",
            num: 4,
        },
        StrToNum {
            str: "five",
            num: 5,
        },
        StrToNum {
            str: "nine",
            num: 9,
        },
    ];
    let five_letter_digits: [StrToNum; 3] = [
        StrToNum {
            str: "three",
            num: 3,
        },
        StrToNum {
            str: "seven",
            num: 7,
        },
        StrToNum {
            str: "eight",
            num: 8,
        },
    ];
    let mut calibration_numbers: Vec<u32> = Vec::with_capacity(input.len());
    for line in input.lines() {
        let mut nums: Vec<u32> = Vec::new();
        for (i, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                let num: u32 = match ch.to_digit(10) {
                    Some(n) => n,
                    None => continue,
                };
                nums.push(num);
            } else if i + 2 < line.len()
                && three_letter_digits
                    .iter()
                    .any(|str_to_num| str_to_num.str == &line[i..=i + 2])
            {
                let num = match three_letter_digits
                    .iter()
                    .find(|str_to_num| str_to_num.str == &line[i..=i + 2])
                    .map(|str_to_num| str_to_num.num)
                {
                    Some(n) => n,
                    None => continue,
                };

                nums.push(num)
            } else if i + 3 < line.len()
                && four_letter_digits
                    .iter()
                    .any(|str_to_num| str_to_num.str == &line[i..=i + 3])
            {
                let num = match four_letter_digits
                    .iter()
                    .find(|str_to_num| str_to_num.str == &line[i..=i + 3])
                    .map(|str_to_num| str_to_num.num)
                {
                    Some(n) => n,
                    None => continue,
                };

                nums.push(num)
            } else if i + 4 < line.len()
                && five_letter_digits
                    .iter()
                    .any(|str_to_num| str_to_num.str == &line[i..=i + 4])
            {
                let num = match five_letter_digits
                    .iter()
                    .find(|str_to_num| str_to_num.str == &line[i..=i + 4])
                    .map(|str_to_num| str_to_num.num)
                {
                    Some(n) => n,
                    None => continue,
                };

                nums.push(num)
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
        let test_input = "two1nine
                        eightwothree
                        abcone2threexyz
                        xtwone3four
                        4nineeightseven2
                        zoneight234
                        7pqrstsixteen";
        let result = part2(test_input);
        assert_eq!(result, 281);
    }
}
