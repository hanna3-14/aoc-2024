use std::collections::HashMap;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut letters: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, letter) in line.chars().enumerate() {
            letters
                .entry(letter)
                .or_default()
                .push((i as i32, j as i32));
        }
    }

    let mut sum: u32 = 0;

    for mut letter in letters.into_iter() {
        while !letter.1.is_empty() {
            let mut region = [letter.1.pop().unwrap()].to_vec();
            let mut i = 0;
            while i < region.len() {
                if letter.1.contains(&(region[i].0 + 1, region[i].1)) {
                    region.push((region[i].0 + 1, region[i].1));
                    letter.1.retain(|position| !region.contains(position));
                }
                if letter.1.contains(&(region[i].0, region[i].1 + 1)) {
                    region.push((region[i].0, region[i].1 + 1));
                    letter.1.retain(|position| !region.contains(position));
                }
                if letter.1.contains(&(region[i].0 - 1, region[i].1)) {
                    region.push((region[i].0 - 1, region[i].1));
                    letter.1.retain(|position| !region.contains(position));
                }
                if letter.1.contains(&(region[i].0, region[i].1 - 1)) {
                    region.push((region[i].0, region[i].1 - 1));
                    letter.1.retain(|position| !region.contains(position));
                }
                i += 1;
            }
            let area = region.len();
            let mut perimeter = 0;
            for position in region.iter() {
                if !region.contains(&(position.0 - 1, position.1)) {
                    perimeter += 1;
                }
                if !region.contains(&(position.0, position.1 - 1)) {
                    perimeter += 1;
                }
                if !region.contains(&(position.0 + 1, position.1)) {
                    perimeter += 1;
                }
                if !region.contains(&(position.0, position.1 + 1)) {
                    perimeter += 1;
                }
            }
            sum += area as u32 * perimeter as u32;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut letters: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, letter) in line.chars().enumerate() {
            letters
                .entry(letter)
                .or_default()
                .push((i as i32, j as i32));
        }
    }

    let mut sum: u32 = 0;

    for mut letter in letters.into_iter() {
        while !letter.1.is_empty() {
            let mut region = [letter.1.pop().unwrap()].to_vec();
            let mut i = 0;
            while i < region.len() {
                if letter.1.contains(&(region[i].0 + 1, region[i].1)) {
                    region.push((region[i].0 + 1, region[i].1));
                    letter.1.retain(|position| !region.contains(position));
                }
                if letter.1.contains(&(region[i].0, region[i].1 + 1)) {
                    region.push((region[i].0, region[i].1 + 1));
                    letter.1.retain(|position| !region.contains(position));
                }
                if letter.1.contains(&(region[i].0 - 1, region[i].1)) {
                    region.push((region[i].0 - 1, region[i].1));
                    letter.1.retain(|position| !region.contains(position));
                }
                if letter.1.contains(&(region[i].0, region[i].1 - 1)) {
                    region.push((region[i].0, region[i].1 - 1));
                    letter.1.retain(|position| !region.contains(position));
                }
                i += 1;
            }
            let area = region.len();
            let mut sides = 0;
            let mut horizontal_fences: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut vertical_fences: HashMap<i32, Vec<i32>> = HashMap::new();
            for position in region.iter() {
                if !region.contains(&(position.0 - 1, position.1)) {
                    horizontal_fences
                        .entry(position.0)
                        .or_default()
                        .push(position.1);
                }
                if !region.contains(&(position.0, position.1 - 1)) {
                    vertical_fences
                        .entry(position.1)
                        .or_default()
                        .push(position.0);
                }
                if !region.contains(&(position.0 + 1, position.1)) {
                    horizontal_fences
                        .entry(position.0 + 1)
                        .or_default()
                        .push(position.1);
                }
                if !region.contains(&(position.0, position.1 + 1)) {
                    vertical_fences
                        .entry(position.1 + 1)
                        .or_default()
                        .push(position.0);
                }
            }
            for mut fence_part in horizontal_fences.into_iter() {
                while !fence_part.1.is_empty() {
                    let mut fence = [fence_part.1.pop().unwrap()].to_vec();
                    let mut j = 0;
                    while j < fence.len() {
                        if fence_part.1.contains(&(fence[j] + 1)) {
                            fence.push(fence[j] + 1);
                            fence_part.1.retain(|f| !fence.contains(f));
                        }
                        if fence_part.1.contains(&(fence[j] - 1)) {
                            fence.push(fence[j] - 1);
                            fence_part.1.retain(|f| !fence.contains(f));
                        }
                        j += 1;
                    }
                    // if two fences cross each other
                    for k in fence.iter() {
                        if vertical_fences
                            .entry(*k)
                            .or_default()
                            .contains(&(fence_part.0 - 1))
                            && vertical_fences
                                .entry(*k)
                                .or_default()
                                .contains(&(fence_part.0))
                        {
                            sides += 2;
                        }
                    }
                    sides += 2;
                }
            }
            sum += area as u32 * sides as u32;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_example2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_example3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1550156));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_example2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_example3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_example4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_example5() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(946084));
    }
}
