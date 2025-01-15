use regex::Regex;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let rex = Regex::new(r"(X\+|X=)(\d+)").ok()?;
    let rey = Regex::new(r"(Y\+|Y=)(\d+)").ok()?;

    let claw_machines: Vec<Vec<(i32, i32)>> = input
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                let x_values: Vec<i32> = rex
                    .captures_iter(line)
                    .filter_map(|caps| caps.get(2)?.as_str().parse::<i32>().ok())
                    .collect();
                let y_values: Vec<i32> = rey
                    .captures_iter(line)
                    .filter_map(|caps| caps.get(2)?.as_str().parse::<i32>().ok())
                    .collect();
                Some((*x_values.first().unwrap(), *y_values.first().unwrap()))
            }
        })
        .collect::<Vec<(i32, i32)>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut tokens = 0;

    for claw_machine in claw_machines.iter() {
        let b = ((claw_machine.get(2).unwrap().0 * claw_machine.first().unwrap().1)
            - claw_machine.get(2).unwrap().1 * claw_machine.first().unwrap().0)
            / ((claw_machine.get(1).unwrap().0 * claw_machine.first().unwrap().1)
                - claw_machine.get(1).unwrap().1 * claw_machine.first().unwrap().0);
        let a = (claw_machine.get(2).unwrap().0 - claw_machine.get(1).unwrap().0 * b)
            / claw_machine.first().unwrap().0;
        if a * claw_machine.first().unwrap().0 + b * claw_machine.get(1).unwrap().0
            == claw_machine.get(2).unwrap().0
            && a * claw_machine.first().unwrap().1 + b * claw_machine.get(1).unwrap().1
                == claw_machine.get(2).unwrap().1
        {
            tokens += a * 3 + b;
        }
    }

    Some(tokens as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rex = Regex::new(r"(X\+)(\d+)").ok()?;
    let rey = Regex::new(r"(Y\+)(\d+)").ok()?;
    let rex_prize = Regex::new(r"(X=)(\d+)").ok()?;
    let rey_prize = Regex::new(r"(Y=)(\d+)").ok()?;

    let claw_machines: Vec<Vec<(i128, i128)>> = input
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                let x_values: Vec<i128> = rex
                    .captures_iter(line)
                    .filter_map(|caps| caps.get(2)?.as_str().parse::<i128>().ok())
                    .collect();
                let y_values: Vec<i128> = rey
                    .captures_iter(line)
                    .filter_map(|caps| caps.get(2)?.as_str().parse::<i128>().ok())
                    .collect();
                let x_values_prize: Vec<i128> = rex_prize
                    .captures_iter(line)
                    .filter_map(|caps| caps.get(2)?.as_str().parse::<i128>().ok())
                    .collect();
                let y_values_prize: Vec<i128> = rey_prize
                    .captures_iter(line)
                    .filter_map(|caps| caps.get(2)?.as_str().parse::<i128>().ok())
                    .collect();
                if !x_values.is_empty() {
                    Some((*x_values.first().unwrap(), *y_values.first().unwrap()))
                } else {
                    Some((
                        *x_values_prize.first().unwrap() + 10000000000000,
                        *y_values_prize.first().unwrap() + 10000000000000,
                    ))
                }
            }
        })
        .collect::<Vec<(i128, i128)>>()
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut tokens = 0;

    for claw_machine in claw_machines.iter() {
        let b = ((claw_machine.get(2).unwrap().0 * claw_machine.first().unwrap().1)
            - claw_machine.get(2).unwrap().1 * claw_machine.first().unwrap().0)
            / ((claw_machine.get(1).unwrap().0 * claw_machine.first().unwrap().1)
                - claw_machine.get(1).unwrap().1 * claw_machine.first().unwrap().0);
        let a = (claw_machine.get(2).unwrap().0 - claw_machine.get(1).unwrap().0 * b)
            / claw_machine.first().unwrap().0;
        if a * claw_machine.first().unwrap().0 + b * claw_machine.get(1).unwrap().0
            == claw_machine.get(2).unwrap().0
            && a * claw_machine.first().unwrap().1 + b * claw_machine.get(1).unwrap().1
                == claw_machine.get(2).unwrap().1
        {
            tokens += a * 3 + b;
        }
    }

    Some(tokens as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(35574));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(80882098756071));
    }
}
