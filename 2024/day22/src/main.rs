use std::{
    collections::{HashMap, LinkedList},
    num::ParseIntError,
};

const DENOM: usize = 16777216;
type DiffKey = (i32, i32, i32, i32);

fn next_secret(current: usize) -> usize {
    let current = (current ^ (current << 6)) % DENOM;
    let current = (current ^ (current >> 5)) % DENOM;
    (current ^ (current << 11)) % DENOM
}

fn secret_after_n(secret: usize) -> usize {
    (0..2000)
        .fold(secret, |prev_secret, _| {
            next_secret(prev_secret)
        })
}

fn make_diffs_dict(secret: usize) -> HashMap<DiffKey, i32> {
    let mut list: LinkedList<i32> = LinkedList::new();
    let mut diffs = HashMap::new();

    let mut last = (secret % 10) as i32;
    let mut secret = secret;
    for _ in 0..2000 {
        secret = next_secret(secret);
        let digit = (secret % 10) as i32;
        let diff = digit - last;
        list.push_back(diff);
        if list.len() == 4 {
            // If 4 diffs in a row, convert to tuple and insert to diff dict
            let mut iter = list.iter_mut();
            let x = (
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
                *iter.next().unwrap(),
            );
            diffs.entry(x).or_insert(digit);
            list.pop_front();
        }

        last = digit;
    }

    diffs
}

fn part_a(puzzle: &str) -> Result<usize, ParseIntError> {
    let secrets: Result<Vec<_>, ParseIntError> = puzzle.lines().map(|line| line.parse()).collect();
    Ok(secrets?.iter().map(|n| secret_after_n(*n)).sum())
}

fn part_b(puzzle: &str) -> Option<i32> {
    let secrets: Vec<usize> = puzzle
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect();

    let mut all_diffs = HashMap::new();
    secrets.iter().for_each(|secret| {
        make_diffs_dict(*secret)
            .iter()
            .for_each(|(key, val)| *all_diffs.entry(*key).or_insert(0) += *val);
    });

    all_diffs.values().max().copied()
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    println!("Part A: {:?}", part_a(puzzle));
    println!("Part B: {:?}", part_b(puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let result = part_a(puzzle);
        assert_eq!(Ok(37327623), result);
    }

    #[test]
    fn test_test_part_b_txt() {
        let puzzle = include_str!("../puzzle/test_part_b.txt");
        let result = part_b(puzzle);
        assert_eq!(Some(23), result);
    }

    // 8685429
    #[test]
    fn test_starting_from_1() {
        let puzzle = "1";
        let result = part_a(puzzle);
        assert_eq!(Ok(8685429), result);
    }

    #[test]
    fn test_secret_number_gen() {
        let mut start = 123;
        for n in [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ] {
            start = next_secret(start);
            assert_eq!(n, start);
        }
    }

    // For the buyer with an initial secret number of 1, changes -2,1,-1,3 first occur when the price is 7
    #[test]
    fn test_diffs_for_1() {
        let diffs = make_diffs_dict(1);
        let expected = Some(7);
        let actual = diffs.get(&(-2, 1, -1, 3)).copied();
        assert_eq!(expected, actual);
    }
}
