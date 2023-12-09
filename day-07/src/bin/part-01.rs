#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);

    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let card_type = vec![("T", 10), ("J", 11), ("Q", 12), ("K", 13), ("A", 14)];
    let mut ranks: Vec<(u32, HashSet<u8>, Vec<u8>, f32)> =
        Vec::with_capacity(input.lines().count());

    for (i, line) in input.lines().enumerate() {
        let hand_and_bid: Vec<&str> = line.split(" ").collect();
        let held_hand = hand_and_bid[0];
        let bid = hand_and_bid[1];
        let mut hand: Vec<u8> = Vec::new();
        let mut unique_hand: HashSet<u8> = HashSet::new();
        let mut occurences = HashMap::new();
        let mut rank;

        for j in held_hand.chars() {
            let num: u8;
            if j.is_numeric() {
                num = j.to_digit(10).unwrap() as u8;
            } else {
                let dig: u8 = match j {
                    'T' => 10_u8,
                    'J' => 11_u8,
                    'Q' => 12_u8,
                    'K' => 13_u8,
                    'A' => 14_u8,
                    _ => panic!("Not a known character"),
                };
                num = dig;
            }
            hand.push(num);
            if unique_hand.get(&num).is_some() {}
            unique_hand.insert(num);
            *occurences.entry(num).or_insert(0) += 1;
        }
        // hand.sort_by(|a, b| b.cmp(&a));
        rank = *occurences.values().max().unwrap() as f32;

        if unique_hand.len() == 5 {
            rank = 0.0;
        }

        if rank == 3.0 {
            if unique_hand.len() == 2 {
                rank += 0.5;
            }
        } else if rank == 2.0 {
            if unique_hand.len() > 3 {
                rank -= 1.0;
            }
        }

        ranks.push((bid.parse::<u32>().unwrap_or(0), unique_hand, hand, rank));
    }

    calc_rank(&mut ranks);

    for i in ranks.iter() {
        println!("{:?}", i)
    }

    ranks
        .iter()
        .enumerate()
        .map(|(index, &(bid, _, _, _))| (index + 1) as u32 * bid)
        .sum()
}

fn calc_rank(cards: &mut Vec<(u32, HashSet<u8>, Vec<u8>, f32)>) -> () {
    cards.sort_by(|a, b| match a.3.total_cmp(&b.3) {
        Ordering::Equal => {
            let mut i = 0;
            loop {
                if i >= a.2.len() {
                    break Ordering::Equal;
                }

                match a.2[i].cmp(&b.2[i]) {
                    Ordering::Equal => {
                        i += 1;
                    }
                    ordering => break ordering,
                }
            }
        }
        ordering => ordering,
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing() {
        let input = include_str!("./test.txt");
        let result = part1(input);

        assert_eq!(result, 6592)
    }
}
