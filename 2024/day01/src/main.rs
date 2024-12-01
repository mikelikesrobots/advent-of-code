use std::collections::HashMap;

fn parse_puzzle(puzzle: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right) = puzzle.lines()
        .map(|s| s
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<_>>())
        .fold( (vec![], vec![]), |(mut left, mut right), line| {
            left.push(line[0]);
            right.push(line[1]);
            (left, right)
        });

    (left, right)
}

fn part_a(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    let sum = left.iter().zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    sum
}

fn part_b(left: &[i32], right: &[i32]) -> i32 {
    let mut right_occurrences: HashMap<i32, i32> = HashMap::new();
    right.iter().for_each(|r| *right_occurrences.entry(*r).or_insert(0) += 1);

    let sum = left.iter()
        .map(|l| l * right_occurrences.get(l).unwrap_or(&0))
        .sum();
    sum
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let (left, right) = parse_puzzle(puzzle);
    let a = part_a(left.clone(), right.clone());
    println!("Part A: {}", a);
    let b = part_b(&left, &right);
    println!("Part B: {}", b);
}
