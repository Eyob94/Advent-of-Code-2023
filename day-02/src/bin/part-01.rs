mod stack;

use std::collections::HashSet;

use stack::Stack;

fn main() {
    let text = include_str!("input1.txt");
    let result = part1(text);

    println!("{}", result)
}

fn part1(games: &str) -> u32 {
    let amount_available: [(&'static str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];
    let mut stack: Stack<u32> = Stack::new();
    let game: Vec<&str> = games.split(&[' ', '\n'][..]).collect();
    let mut available_games = HashSet::new();
    let mut impossible_games = HashSet::new();

    for letter in game.iter() {
        let mut letter = letter.to_string();
        letter.retain(|c| !r#"(),".;:'"#.contains(c));
        if letter.is_empty() {
            break;
        }
        if letter.chars().nth(0).unwrap().is_numeric() {
            let num: u32 = match letter.parse::<u32>() {
                Ok(n) => n,
                Err(_) => continue,
            };
            stack.add(num);
        } else if letter.chars().nth(0).unwrap().is_alphabetic() {
            let last_amount = match stack.pop() {
                Some(n) => n,
                None => continue,
            };

            let this_game = match stack.peek() {
                Some(n) => *n,
                None => continue,
            };
            available_games.insert(this_game);

            if let Some(index) = amount_available
                .map(|colors| colors.0.to_string())
                .iter()
                .position(|x| x == &letter || x == &letter[0..letter.len() - 2])
            {
                if last_amount > amount_available[index].1 {
                    impossible_games.insert(this_game);
                }
            }
        }
    }

    available_games
        .difference(&impossible_games)
        .into_iter()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hope_it_works() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 1 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1(test_data);

        assert_eq!(result, 8);
    }
}
