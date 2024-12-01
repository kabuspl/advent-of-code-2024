use std::collections::HashMap;

static INPUT: &'static str = include_str!("../../inputs/01.txt");

fn main() {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    let mut right_occurrences: HashMap<i32, i32> = HashMap::new();

    for line in INPUT.lines() {
        if line.is_empty() {
            continue;
        }

        let numbers: Vec<i32> = line
            .split("   ")
            .map(|number| number.parse().unwrap())
            .collect();

        left_list.push(numbers[0]);
        right_list.push(numbers[1]);

        right_occurrences
            .entry(numbers[1])
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    }

    left_list.sort();
    right_list.sort();

    let mut diff_sum = 0;
    let mut similarity_score_total = 0;

    for (left_number, right_number) in left_list.iter().zip(right_list) {
        let diff = (left_number - right_number).abs();
        diff_sum += diff;

        let similarity_score = left_number * right_occurrences.get(left_number).unwrap_or(&0);

        similarity_score_total += similarity_score;
    }

    println!("Total distance between lists: {diff_sum}");
    println!("Total similairty score: {similarity_score_total}");
}
