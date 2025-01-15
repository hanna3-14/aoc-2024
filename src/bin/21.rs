use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let numeric_keypad: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (3, 2)),
        ('0', (3, 1)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);

    let directional_keypad: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (0, 2)),
        ('^', (0, 1)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    let mut result = 0;
    let mut current_key: char = 'A';
    for &line in lines.iter() {
        let mut final_sequence = String::new();
        for next_key in line.chars() {
            let numeric_sequence: String =
                use_numeric_keypad(current_key, next_key, &numeric_keypad);
            let first_directional_sequence =
                use_directional_keypad(numeric_sequence, &directional_keypad);
            let second_directional_sequence =
                use_directional_keypad(first_directional_sequence, &directional_keypad);
            final_sequence.push_str(&second_directional_sequence);
            current_key = next_key;
        }
        let mut number = String::from(line);
        number.pop();
        let pin = number.parse::<usize>().unwrap();

        result += pin * final_sequence.len();
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    let numeric_keypad: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (3, 2)),
        ('0', (3, 1)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
    ]);

    let directional_keypad: HashMap<char, (i32, i32)> = HashMap::from([
        ('A', (0, 2)),
        ('^', (0, 1)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);

    let mut result = 0;
    let mut current_key: char = 'A';
    let mut database: HashMap<(String, usize), usize> = HashMap::new();
    for &line in lines.iter() {
        let mut final_sequence = 0;
        for next_key in line.chars() {
            let numeric_sequence: String =
                use_numeric_keypad(current_key, next_key, &numeric_keypad);

            let length_after_25_keypads = recursive_directional_keypad(
                numeric_sequence.as_str(),
                0,
                25,
                &directional_keypad,
                &mut database,
            );

            final_sequence += length_after_25_keypads;
            current_key = next_key;
        }

        let mut number = String::from(line);
        number.pop();
        let pin = number.parse::<usize>().unwrap();

        result += pin * final_sequence;
    }

    Some(result as u64)
}

pub fn recursive_directional_keypad(
    sequence: &str,
    i: usize,
    number_of_directional_keypads: usize,
    directional_keypad: &HashMap<char, (i32, i32)>,
    database: &mut HashMap<(String, usize), usize>,
) -> usize {
    if let Some(&cached_result) = database.get(&(sequence.to_string(), i)) {
        return cached_result;
    }

    if i == number_of_directional_keypads {
        let result = sequence.chars().filter(|&c| c != ':').count();
        return result;
    }

    let mut splits: Vec<_> = sequence.split('A').collect();
    let mut result = 0;

    splits.pop();

    for split in splits {
        let mut part = String::from(split);
        part.push('A');

        let part_directed = use_directional_keypad(part.clone(), directional_keypad);

        let new_number = recursive_directional_keypad(
            &part_directed,
            i + 1,
            number_of_directional_keypads,
            directional_keypad,
            database,
        );

        result += new_number;

        database.insert((part, i + 1), new_number);
    }

    database.insert((sequence.to_string(), i), result);
    result
}

pub fn use_numeric_keypad(
    current_key: char,
    next_key: char,
    numeric_keypad: &HashMap<char, (i32, i32)>,
) -> String {
    let current_position = *numeric_keypad.get(&current_key).unwrap();
    let next_position = *numeric_keypad.get(&next_key).unwrap();
    let mut numeric_sequence = String::new();
    let mut moves_up = 0;
    if next_position.0 - current_position.0 < 0 {
        moves_up = (next_position.0 - current_position.0).abs();
    }
    let moves_down = next_position.0 - current_position.0;
    let mut moves_to_the_left = 0;
    if next_position.1 - current_position.1 < 0 {
        moves_to_the_left = (next_position.1 - current_position.1).abs();
    }
    let moves_to_the_right = next_position.1 - current_position.1;

    if current_key == '0' || current_key == 'A' && moves_to_the_left > 1 {
        for _ in 0..moves_up {
            numeric_sequence.push('^');
        }
        for _ in 0..moves_to_the_left {
            numeric_sequence.push('<');
        }
    } else {
        for _ in 0..moves_to_the_left {
            numeric_sequence.push('<');
        }
        for _ in 0..moves_up {
            numeric_sequence.push('^');
        }
    }
    if current_key == '1'
        || current_key == '4' && moves_down > 1
        || current_key == '7' && moves_down > 2
    {
        for _ in 0..moves_to_the_right {
            numeric_sequence.push('>');
        }
        for _ in 0..moves_down {
            numeric_sequence.push('v');
        }
    } else {
        for _ in 0..moves_down {
            numeric_sequence.push('v');
        }
        for _ in 0..moves_to_the_right {
            numeric_sequence.push('>');
        }
    }
    numeric_sequence.push('A');
    numeric_sequence
}

pub fn use_directional_keypad(
    sequence: String,
    directional_keypad: &HashMap<char, (i32, i32)>,
) -> String {
    let mut directional_sequence = String::new();
    let mut current_key = 'A';

    for next_key in sequence.chars() {
        let current_position = *directional_keypad.get(&current_key).unwrap();
        let next_position = *directional_keypad.get(&next_key).unwrap();
        let mut moves_up = 0;
        if next_position.0 - current_position.0 < 0 {
            moves_up = (next_position.0 - current_position.0).abs();
        }
        let moves_down = next_position.0 - current_position.0;
        let mut moves_to_the_left = 0;
        if next_position.1 - current_position.1 < 0 {
            moves_to_the_left = (next_position.1 - current_position.1).abs();
        }
        let moves_to_the_right = next_position.1 - current_position.1;

        if current_key == '^' || current_key == 'A' && moves_to_the_left > 1 {
            for _ in 0..moves_down {
                directional_sequence.push('v');
            }
            for _ in 0..moves_to_the_left {
                directional_sequence.push('<');
            }
        } else {
            for _ in 0..moves_to_the_left {
                directional_sequence.push('<');
            }
            for _ in 0..moves_down {
                directional_sequence.push('v');
            }
        }
        if current_key == '<' {
            for _ in 0..moves_to_the_right {
                directional_sequence.push('>');
            }
            for _ in 0..moves_up {
                directional_sequence.push('^');
            }
        } else {
            for _ in 0..moves_up {
                directional_sequence.push('^');
            }
            for _ in 0..moves_to_the_right {
                directional_sequence.push('>');
            }
        }
        directional_sequence.push('A');
        current_key = next_key;
    }
    directional_sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(217662));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(263617786809000));
    }
}
