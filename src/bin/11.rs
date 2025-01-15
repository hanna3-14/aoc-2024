use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<i64> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let number_of_blinks = 25;

    let mut database: HashMap<(i64, i32), u64> = HashMap::new();

    let stone_counter: u64 = numbers
        .iter()
        .map(|&number| count_stones(number, 1, number_of_blinks, &mut database))
        .sum();

    Some(stone_counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let numbers: Vec<i64> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let number_of_blinks = 75;

    let mut database: HashMap<(i64, i32), u64> = HashMap::new();

    let stone_counter: u64 = numbers
        .iter()
        .map(|&number| count_stones(number, 1, number_of_blinks, &mut database))
        .sum();

    Some(stone_counter)
}

pub fn count_stones(
    mut number: i64,
    mut i: i32,
    number_of_blinks: i32,
    database: &mut HashMap<(i64, i32), u64>,
) -> u64 {
    if let Some(&result) = database.get(&(number, i)) {
        return result;
    }

    let mut result: u64 = 0;

    while i <= number_of_blinks {
        if i == number_of_blinks {
            if number.to_string().len() % 2 == 0 {
                result += 2;
            } else {
                result += 1;
            }
            break;
        }
        if number == 0 {
            number = 1;
        } else if number.to_string().len() % 2 == 0 {
            let base: i32 = 10;
            let divisor = base.pow(number.to_string().len() as u32 / 2) as i64;
            result += count_stones(number / divisor, i + 1, number_of_blinks, database)
                + count_stones(number % divisor, i + 1, number_of_blinks, database);
            break;
        } else {
            number *= 2024;
        }
        i += 1;
    }

    database.insert((number, i), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(209412));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(248967696501656));
    }
}
