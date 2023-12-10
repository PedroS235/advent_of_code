fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn next_in_squence(values: Vec<i64>) -> i64 {
    let mut sequences = Vec::new();
    let mut sequence = Vec::new();
    let mut not_found = true;

    let mut next_values = values.clone();

    while not_found {
        for (i, v) in next_values.iter().enumerate().skip(1) {
            let diff = v - next_values[i - 1];
            sequence.push(diff);
        }

        if sequence.iter().all(|&v| v == 0) {
            not_found = false;
        }
        sequences.push(sequence.clone());
        next_values = sequence.clone();
        sequence.clear();
    }

    for i in (0..sequences.len() - 1).rev() {
        let prev = sequences[i + 1].last().unwrap();
        let current = sequences[i].last().unwrap();
        let next = prev + current;
        sequences[i].push(next);
    }

    values.last().unwrap().clone() + sequences[0].last().unwrap().clone()
}

fn part_1(input: &str) -> i64 {
    let values = input
        .lines()
        .map(|line| {
            line.split(' ')
                .into_iter()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    values.iter().map(|seq| next_in_squence(seq.clone())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = part_1(input);
        assert_eq!(result, 114)
    }
}
