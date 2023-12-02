fn main() {
    let text = include_str!("input.txt");
    let result = part1(text);

    println!("{}", result)
}

fn part1(games: &str) -> u32 {
    let game: Vec<&str> = games.split(&[' ', '\n'][..]).collect();
    let colors = ["red", "green", "blue"];
    let mut powers: Vec<u32> = Vec::with_capacity(games.lines().count());
    let mut line_power: Vec<u32> = vec![1, 1, 1];
    let mut game_number: bool = false;

    for (i, letter) in game.iter().enumerate() {
        let mut letter = letter.to_string();
        letter.retain(|c| !r#"(),".;:'"#.contains(c));
        if letter.is_empty() {
            break;
        } else if letter == "Game" {
            game_number = true;
        } else if letter.chars().nth(0).unwrap().is_numeric() {
            if game_number {
                let power = line_power.iter().fold(1, |acc, e| acc * e);
                powers.push(power);
                line_power = vec![1, 1, 1];
                game_number = false;
                continue;
            }
            let mut color = game[i + 1].to_string();
            color.retain(|c| !r#"(),".;:'"#.contains(c));

            let color_index: usize = match colors.iter().position(|&c| c == color) {
                Some(n) => n,
                None => continue,
            };

            let num: u32 = match letter.parse::<u32>() {
                Ok(n) => n,
                Err(_) => continue,
            };
            if num > line_power[color_index] {
                line_power[color_index] = num;
            }
        }
    }

    let power = line_power.iter().fold(1, |acc, e| acc * e);
    powers.push(power);



    powers[1..].iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hope_it_works2() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1(test_data);

        assert_eq!(result, 2286);
    }
}
