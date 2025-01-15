use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let mut result = 0;

    let mut number: u64;

    for line in lines.iter() {
        number = line.parse::<u64>().unwrap();
        for _ in 0..2000 {
            number = calculate_next_secret_number(number);
        }
        result += number;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut result: u64 = 0;

    let mut numbers_of_all_lines: Vec<Vec<u8>> = vec![];
    let mut differences_of_all_lines: Vec<Vec<i32>> = vec![];

    for line in lines.iter() {
        let mut numbers: Vec<u8> = vec![];
        let mut differences: Vec<i32> = vec![];
        let mut number = line.parse::<u64>().unwrap();
        for i in 0..2000 {
            if i == 0 {
                numbers.push((number % 10) as u8);
            }
            number = calculate_next_secret_number(number);
            numbers.push((number % 10) as u8);
            if i >= 1 {
                differences.push(numbers[i] as i32 - numbers[i - 1] as i32);
            }
        }
        numbers_of_all_lines.push(numbers);
        differences_of_all_lines.push(differences);
    }

    let mut banana_filters: Vec<&[i32]> = Vec::new();

    for differences in differences_of_all_lines.iter() {
        for banana_filter in differences.windows(4) {
            banana_filters.push(banana_filter);
        }
    }

    for &banana_filter in banana_filters.iter().unique() {
        let current_result = numbers_of_all_lines
            .par_iter()
            .enumerate()
            .map(|(i, numbers)| {
                let banana_positions: Vec<_> = differences_of_all_lines[i]
                    .windows(4)
                    .filter(|&chunk| chunk == banana_filter)
                    .collect();
                if !banana_positions.is_empty() {
                    let banana_indices = differences_of_all_lines[i]
                        .windows(4)
                        .find_position(|&chunk| chunk == banana_filter);
                    let banana_index = banana_indices.unwrap();
                    numbers[banana_index.0 + 4] as u64
                } else {
                    0
                }
            })
            .sum();
        if current_result > result {
            result = current_result;
        }
    }

    Some(result)
}

pub fn calculate_next_secret_number(current_number: u64) -> u64 {
    let mut mixed_number = prune(mix(current_number * 64, current_number));
    mixed_number = prune(mix(mixed_number / 32, mixed_number));
    mixed_number = prune(mix(mixed_number * 2048, mixed_number));
    mixed_number
}

pub fn mix(given_value: u64, secret_number: u64) -> u64 {
    given_value ^ secret_number
}

pub fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(15006633487));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1710));
    }
}
