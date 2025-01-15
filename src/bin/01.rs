advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = read_input(input);

    left_list.sort_unstable(); // sort_unstable doesn't maintain the relative order of equal
    right_list.sort_unstable();

    let difference: i32 = left_list
        .iter()
        .zip(right_list.iter()) // pair elements from both lists
        .map(|(l, r)| (l - r).abs()) // calculate absolute differences
        .sum();

    Some(difference as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = read_input(input);

    // precompute the frequency of each element in right_list
    let mut right_counts = std::collections::HashMap::new();
    for &num in &right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    let difference: i32 = left_list
        .iter()
        .map(|&l| {
            let count = *right_counts.get(&l).unwrap_or(&0);
            l * count
        })
        .sum();

    Some(difference as u32)
}

pub fn read_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }

    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2367773));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(21271939));
    }
}
