use std::collections::BTreeMap;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut numbers = line
        .split(": ")
        .last()
        .expect("Supposed to be winning number and numbers")
        .split(" | ");

    (
        numbers
            .next()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect(),
        numbers
            .next()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect(),
    )
}

fn part_1(input: &str) -> u32 {
    // Convert input to a matrix of chars
    let lines = input.lines();

    let mut card_hash: BTreeMap<u32, (u32, u32)> = BTreeMap::new();

    for (i, line) in lines.enumerate() {
        let mut card_sum = 0;
        let (winning_numbers, numbers) = parse_line(line);
        let hash_numbers: HashMap<u32, u32> = numbers.iter().map(|n| (*n, 0 as u32)).collect();

        for win_n in winning_numbers {
            if hash_numbers.contains_key(&win_n) {
                card_sum += 1;
            }
        }
        if card_hash.contains_key(&(i as u32)) {
            let (card_count, _) = card_hash.get(&(i as u32)).unwrap();
            card_hash.insert(i as u32, (card_count + 1, card_sum));
        } else {
            card_hash.insert(i as u32, (1, card_sum));
        }

        for j in 1..card_sum + 1 {
            if card_hash.contains_key(&(i as u32 + j as u32)) {
                let (card_count, _) = card_hash.get(&(i as u32 + j as u32)).unwrap();
                card_hash.insert(
                    i as u32 + j as u32,
                    (card_count + card_hash.get(&(i as u32)).unwrap().0, card_sum),
                );
            } else {
                card_hash.insert(
                    i as u32 + j as u32,
                    (card_hash.get(&(i as u32)).unwrap().0, card_sum),
                );
            }
        }
    }

    let mut sum = 0;

    for i in card_hash.values() {
        sum += i.0;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part_1(input);
        assert_eq!(result, 30)
    }
}
