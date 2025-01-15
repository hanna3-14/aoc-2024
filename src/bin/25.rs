use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let (locks, keys) = read_locks_and_keys(input);

    let mut locks_map: Vec<HashMap<usize, usize>> = Vec::new();

    for lock in locks.iter() {
        let mut lock_map: HashMap<usize, usize> = HashMap::new();
        for i in (0..lock.len()).rev() {
            let positions: Vec<_> = lock[i].chars().positions(|char| char == '#').collect();
            for position in positions.iter() {
                if !lock_map.contains_key(position) {
                    lock_map.insert(*position, i);
                }
            }
        }
        locks_map.push(lock_map);
    }

    let mut keys_map: Vec<HashMap<usize, usize>> = Vec::new();

    for key in keys.iter() {
        let mut key_map: HashMap<usize, usize> = HashMap::new();
        for (i, line) in key.iter().enumerate() {
            let positions: Vec<_> = line.chars().positions(|char| char == '#').collect();
            for position in positions.iter() {
                if !key_map.contains_key(position) {
                    key_map.insert(*position, 6 - i);
                }
            }
        }
        keys_map.push(key_map);
    }

    let mut match_counter = 0;

    for key in keys_map.iter() {
        for lock in locks_map.iter() {
            let mut matching_string = String::new();
            for i in 0..key.len() {
                if key.get(&i).unwrap() + lock.get(&i).unwrap() > 5 {
                    break;
                } else {
                    matching_string.push('1');
                }
            }
            if matching_string.len() == key.len() {
                match_counter += 1;
            }
        }
    }

    Some(match_counter)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

pub fn read_locks_and_keys(input: &str) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let mut locks: Vec<Vec<&str>> = Vec::new();
    let mut keys: Vec<Vec<&str>> = Vec::new();

    let locks_and_keys: Vec<Vec<&str>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|chunk| chunk.to_vec())
        .collect();

    for entry in locks_and_keys.iter() {
        if entry.first().unwrap().eq(&"#####") {
            locks.push(entry.to_vec());
        } else {
            keys.push(entry.to_vec());
        }
    }

    (locks, keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(3264));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
