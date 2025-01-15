use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if check_if_report_is_save(&numbers) {
            safe += 1
        }
    }

    Some(safe as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;

    for line in input.lines() {
        let mut numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        let mut report_is_save: bool = false;
        let mut i = 0;

        if check_if_report_is_save(&numbers) {
            report_is_save = true;
        } else {
            while !report_is_save {
                let number: i32 = numbers[i];
                numbers.remove(i);
                report_is_save = check_if_report_is_save(&numbers);
                numbers.insert(i, number);
                i += 1;
                if i >= numbers.len() {
                    break;
                }
            }
        }

        if report_is_save {
            safe += 1
        }
    }

    Some(safe as u32)
}

fn check_if_report_is_save(numbers: &[i32]) -> bool {
    let mut differences: Vec<i32> = vec![];

    for (prev, next) in numbers.iter().tuple_windows() {
        differences.push(next - prev);
    }

    if (differences.iter().all(|item| item.is_positive())
        || differences.iter().all(|item| item.is_negative()))
        && differences.iter().all(|item| item.abs() < 4)
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(585));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(626));
    }
}
