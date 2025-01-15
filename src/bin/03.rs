use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul)\((\d+),(\d+)\)").ok()?;

    let result = input
        .lines()
        .flat_map(|line| {
            re.captures_iter(line).filter_map(|caps| {
                Some(
                    caps.get(2)?.as_str().parse::<u32>().ok()?
                        * caps.get(3)?.as_str().parse::<u32>().ok()?,
                )
            })
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    let mut do_multiply = true;

    let re = Regex::new(r"(mul)\((\d+),(\d+)\)|(do)(\()(\))|(don't)(\()(\))").ok()?;

    for line in input.lines() {
        for (_, [operator, op1, op2]) in re.captures_iter(line).map(|caps| caps.extract()) {
            if do_multiply && operator.eq("mul") {
                result += op1.parse::<u32>().ok()? * op2.parse::<u32>().ok()?;
            } else if operator.eq("don't") {
                do_multiply = false;
            } else if operator.eq("do") {
                do_multiply = true;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(171183089));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(63866497));
    }
}
