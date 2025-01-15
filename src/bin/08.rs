use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut positions_of_antinotes: Vec<(i32, i32)> = vec![];

    for letter in b'A'..=b'Z' {
        let antinotes_for_letter = find_antinotes_for_letter(&lines, letter);
        if antinotes_for_letter.is_some() {
            let unwrapped = antinotes_for_letter.unwrap();
            for position in unwrapped.iter() {
                positions_of_antinotes.push(*position);
            }
        }
    }

    for letter in b'a'..=b'z' {
        let antinotes_for_letter = find_antinotes_for_letter(&lines, letter);
        if antinotes_for_letter.is_some() {
            let unwrapped = antinotes_for_letter.unwrap();
            for position in unwrapped.iter() {
                positions_of_antinotes.push(*position);
            }
        }
    }

    for letter in b'0'..=b'9' {
        let antinotes_for_letter = find_antinotes_for_letter(&lines, letter);
        if antinotes_for_letter.is_some() {
            let unwrapped = antinotes_for_letter.unwrap();
            for position in unwrapped.iter() {
                positions_of_antinotes.push(*position);
            }
        }
    }

    Some(positions_of_antinotes.iter().unique().count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut positions_of_antinotes: Vec<(i32, i32)> = vec![];

    for letter in b'A'..=b'Z' {
        let antinotes_for_letter = find_antinotes_for_letter_part_2(&lines, letter);
        if antinotes_for_letter.is_some() {
            let unwrapped = antinotes_for_letter.unwrap();
            for position in unwrapped.iter() {
                positions_of_antinotes.push(*position);
            }
        }
    }

    for letter in b'a'..=b'z' {
        let antinotes_for_letter = find_antinotes_for_letter_part_2(&lines, letter);
        if antinotes_for_letter.is_some() {
            let unwrapped = antinotes_for_letter.unwrap();
            for position in unwrapped.iter() {
                positions_of_antinotes.push(*position);
            }
        }
    }

    for letter in b'0'..=b'9' {
        let antinotes_for_letter = find_antinotes_for_letter_part_2(&lines, letter);
        if antinotes_for_letter.is_some() {
            let unwrapped = antinotes_for_letter.unwrap();
            for position in unwrapped.iter() {
                positions_of_antinotes.push(*position);
            }
        }
    }

    Some(positions_of_antinotes.iter().unique().count() as u32)
}

pub fn find_antinotes_for_letter(lines: &[&str], letter: u8) -> Option<Vec<(i32, i32)>> {
    let number_of_rows = lines.len() as i32;
    let number_of_columns = lines.first()?.len() as i32;

    let mut positions_of_letter: Vec<(usize, usize)> = vec![];

    for (i, line) in lines.iter().enumerate() {
        let columns = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == letter as char)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for column in columns.iter() {
            positions_of_letter.push((i, *column));
        }
    }

    if positions_of_letter.is_empty() {
        return None;
    }

    let mut positions_of_antinotes: Vec<(i32, i32)> = vec![];

    for i in 0..positions_of_letter.len() - 1 {
        for j in i + 1..positions_of_letter.len() {
            let x: i32 = positions_of_letter[j].0 as i32 - positions_of_letter[i].0 as i32;
            let y: i32 = positions_of_letter[j].1 as i32 - positions_of_letter[i].1 as i32;
            if positions_of_letter[i].0 as i32 - x >= 0
                && positions_of_letter[i].1 as i32 - y >= 0
                && positions_of_letter[i].1 as i32 - y < number_of_columns
            {
                positions_of_antinotes.push((
                    positions_of_letter[i].0 as i32 - x,
                    positions_of_letter[i].1 as i32 - y,
                ));
            }
            if positions_of_letter[j].0 as i32 + x < number_of_rows
                && positions_of_letter[j].1 as i32 + y < number_of_columns
                && positions_of_letter[j].1 as i32 + y >= 0
            {
                positions_of_antinotes.push((
                    positions_of_letter[j].0 as i32 + x,
                    positions_of_letter[j].1 as i32 + y,
                ));
            }
        }
    }

    Some(positions_of_antinotes)
}

pub fn find_antinotes_for_letter_part_2(lines: &[&str], letter: u8) -> Option<Vec<(i32, i32)>> {
    let number_of_rows = lines.len() as i32;
    let number_of_columns = lines.first()?.len() as i32;

    let mut positions_of_letter: Vec<(usize, usize)> = vec![];

    for (i, line) in lines.iter().enumerate() {
        let columns = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == letter as char)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for column in columns.iter() {
            positions_of_letter.push((i, *column));
        }
    }

    if positions_of_letter.is_empty() {
        return None;
    }

    let mut positions_of_antinotes: Vec<(i32, i32)> = vec![];

    for i in 0..positions_of_letter.len() - 1 {
        for j in i + 1..positions_of_letter.len() {
            let x: i32 = positions_of_letter[j].0 as i32 - positions_of_letter[i].0 as i32;
            let y: i32 = positions_of_letter[j].1 as i32 - positions_of_letter[i].1 as i32;
            let mut factor = 0;
            while positions_of_letter[i].0 as i32 - factor * x >= 0
                && positions_of_letter[i].1 as i32 - factor * y >= 0
                && positions_of_letter[i].1 as i32 - factor * y < number_of_columns
            {
                positions_of_antinotes.push((
                    positions_of_letter[i].0 as i32 - factor * x,
                    positions_of_letter[i].1 as i32 - factor * y,
                ));
                factor += 1;
            }
            factor = 0;
            while positions_of_letter[j].0 as i32 + factor * x < number_of_rows
                && positions_of_letter[j].0 as i32 + factor * x >= 0
                && positions_of_letter[j].1 as i32 + factor * y < number_of_columns
                && positions_of_letter[j].1 as i32 + factor * y >= 0
            {
                positions_of_antinotes.push((
                    positions_of_letter[j].0 as i32 + factor * x,
                    positions_of_letter[j].1 as i32 + factor * y,
                ));
                factor += 1;
            }
        }
    }

    Some(positions_of_antinotes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(396));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1200));
    }
}
