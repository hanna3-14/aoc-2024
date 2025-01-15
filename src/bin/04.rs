use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let number_of_rows = lines.len();
    let number_of_columns = lines.first()?.len();

    // collect positions of X
    let positions_of_x: HashMap<usize, Vec<usize>> = lines
        .par_iter()
        .enumerate()
        .map(|(index, line)| {
            (
                index,
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == 'X')
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let result: u32 = positions_of_x
        .par_iter()
        .map(|(&i, columns)| {
            let mut local_result = 0;
            for &j in columns {
                // check XMAS horizontally
                if j < number_of_columns - 3
                    && lines[i].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i].chars().nth(j + 2).unwrap() == 'A'
                    && lines[i].chars().nth(j + 3).unwrap() == 'S'
                {
                    local_result += 1;
                }

                // check SAMX horizontally
                if j >= 3
                    && lines[i].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i].chars().nth(j - 2).unwrap() == 'A'
                    && lines[i].chars().nth(j - 3).unwrap() == 'S'
                {
                    local_result += 1;
                }

                // check XMAS vertically
                if i < number_of_rows - 3
                    && lines[i + 1].chars().nth(j).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j).unwrap() == 'S'
                {
                    local_result += 1;
                }

                // check SAMX vertically
                if i >= 3
                    && lines[i - 1].chars().nth(j).unwrap() == 'M'
                    && lines[i - 2].chars().nth(j).unwrap() == 'A'
                    && lines[i - 3].chars().nth(j).unwrap() == 'S'
                {
                    local_result += 1;
                }

                // check XMAS diagonal from upper left to lower right
                if i < number_of_rows - 3
                    && j < number_of_columns - 3
                    && lines[i + 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j + 2).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j + 3).unwrap() == 'S'
                {
                    local_result += 1;
                }

                // check SAMX diagonal from upper left to lower right
                if i >= 3
                    && j >= 3
                    && lines[i - 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i - 2].chars().nth(j - 2).unwrap() == 'A'
                    && lines[i - 3].chars().nth(j - 3).unwrap() == 'S'
                {
                    local_result += 1;
                }

                // check XMAS diagonal from lower left to upper right
                if i >= 3
                    && j < number_of_columns - 3
                    && lines[i - 1].chars().nth(j + 1).unwrap() == 'M'
                    && lines[i - 2].chars().nth(j + 2).unwrap() == 'A'
                    && lines[i - 3].chars().nth(j + 3).unwrap() == 'S'
                {
                    local_result += 1;
                }

                // check SAMX diagonal from lower left to upper right
                if i < number_of_rows - 3
                    && j >= 3
                    && lines[i + 1].chars().nth(j - 1).unwrap() == 'M'
                    && lines[i + 2].chars().nth(j - 2).unwrap() == 'A'
                    && lines[i + 3].chars().nth(j - 3).unwrap() == 'S'
                {
                    local_result += 1;
                }
            }
            local_result
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let number_of_rows = lines.len();

    let positions_of_a: HashMap<usize, Vec<usize>> = (1..number_of_rows - 1)
        .into_par_iter()
        .map(|index| {
            (
                index,
                lines[index]
                    .chars()
                    .enumerate()
                    .filter(|&(i, c)| c == 'A' && i > 0 && i < number_of_rows - 1)
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let result: u32 = positions_of_a
        .par_iter()
        .map(|(&i, columns)| {
            columns
                .iter()
                .filter(|&&j| {
                    let upper_left = lines[i - 1].chars().nth(j - 1).unwrap();
                    let lower_right = lines[i + 1].chars().nth(j + 1).unwrap();
                    let upper_right = lines[i - 1].chars().nth(j + 1).unwrap();
                    let lower_left = lines[i + 1].chars().nth(j - 1).unwrap();

                    (upper_left == 'M' && lower_right == 'S'
                        || upper_left == 'S' && lower_right == 'M')
                        && (upper_right == 'M' && lower_left == 'S'
                            || upper_right == 'S' && lower_left == 'M')
                })
                .count() as u32
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2571));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1992));
    }
}
