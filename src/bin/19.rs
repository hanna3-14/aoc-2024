use std::cmp;
use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut design_counter = 0;

    let lines: Vec<&str> = input.lines().collect();

    let mut towels: Vec<&str> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|towel| towel.trim())
        .collect();

    towels.sort_by_key(|towel| towel.len());
    let max_towels_len = towels.last().unwrap().len();

    let designs: Vec<&str> = lines.iter().skip(2).map(|&line| line.trim()).collect();

    let mut database: HashMap<String, bool> = HashMap::new();

    for design in designs.iter() {
        if design_is_possible(design, &towels, max_towels_len, &mut database) {
            design_counter += 1;
        }
    }

    Some(design_counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let mut towels: Vec<&str> = lines
        .first()
        .unwrap()
        .split(',')
        .map(|towel| towel.trim())
        .collect();

    towels.sort_by_key(|towel| towel.len());
    let max_towels_len = towels.last().unwrap().len();

    let designs: Vec<&str> = lines.iter().skip(2).map(|&line| line.trim()).collect();

    let mut database: HashMap<String, u64> = HashMap::new();

    let mut design_counter = 0;

    for design in designs.iter() {
        design_counter += count_possibilities(design, &towels, max_towels_len, &mut database);
    }

    Some(design_counter)
}

pub fn design_is_possible(
    design: &str,
    towels: &[&str],
    max_towels_len: usize,
    database: &mut HashMap<String, bool>,
) -> bool {
    if design.eq("") {
        return true;
    }
    if database.contains_key(design) {
        return *database.get(design).unwrap();
    }
    for i in 0..cmp::min(design.len(), max_towels_len) + 1 {
        if towels.contains(&&design[..i])
            && design_is_possible(&design[i..], towels, max_towels_len, database)
        {
            database.insert(design.to_string(), true);
            return true;
        }
    }
    database.insert(design.to_string(), false);
    false
}

pub fn count_possibilities(
    design: &str,
    towels: &[&str],
    max_towels_len: usize,
    database: &mut HashMap<String, u64>,
) -> u64 {
    if design.eq("") {
        return 1;
    }
    let mut count = 0;
    if database.contains_key(design) {
        return *database.get(design).unwrap();
    }
    for i in 0..cmp::min(design.len(), max_towels_len) + 1 {
        if towels.contains(&&design[..i]) {
            count += count_possibilities(&design[i..], towels, max_towels_len, database);
            database.insert(design.to_string(), count);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(319));
    }

    #[test]
    fn test_part_two_brwrr() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_bggr() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_gbbr() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_rrbgbr() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_ubwu() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two_bwurrg() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_brgr() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 8,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_bbrgwb() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 9,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(692575723305545));
    }
}
