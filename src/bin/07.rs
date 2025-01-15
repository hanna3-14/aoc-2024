advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u128> {
    let mut sum: u128 = 0;

    let equations: Vec<&str> = input.lines().collect();

    for equation in equations.iter() {
        let numbers: Vec<u128> = equation
            .replace(":", "")
            .split_whitespace()
            .filter_map(|x| x.parse::<u128>().ok())
            .collect();

        if equation_is_true(&numbers) {
            sum += numbers[0];
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut sum: u128 = 0;

    let equations: Vec<&str> = input.lines().collect();

    for equation in equations.iter() {
        let numbers: Vec<u128> = equation
            .replace(":", "")
            .split_whitespace()
            .filter_map(|x| x.parse::<u128>().ok())
            .collect();

        if equation_is_true_with_concatination(&numbers) {
            sum += numbers[0];
        }
    }

    Some(sum)
}

pub fn equation_is_true(numbers: &Vec<u128>) -> bool {
    let mut lel: usize = 2;
    let operators: Vec<fn(u128, u128) -> u128> = vec![add, multiply];
    tree_traversal(numbers, &mut lel, numbers[1], &operators)
}

pub fn tree_traversal(
    numbers: &Vec<u128>,
    i: &mut usize,
    partial: u128,
    operators: &Vec<fn(u128, u128) -> u128>,
) -> bool {
    if partial > numbers[0] {
        return false;
    } else if *i == numbers.len() {
        return partial == numbers[0];
    } else {
        let mut i_plus = *i;
        let mut i_mul = *i;
        for (op_counter, operator) in operators.iter().enumerate() {
            let result = operator(partial, numbers[*i]);
            if op_counter == 0 {
                i_plus += 1;
                if tree_traversal(numbers, &mut i_plus, result, operators) {
                    return true;
                } else {
                    continue;
                }
            } else {
                i_mul += 1;
                if tree_traversal(numbers, &mut i_mul, result, operators) {
                    return true;
                } else {
                    continue;
                }
            }
        }
    }
    false
}

pub fn equation_is_true_with_concatination(numbers: &Vec<u128>) -> bool {
    let mut lel: usize = 2;
    let operators: Vec<fn(u128, u128) -> u128> = vec![add, multiply, concat];
    tree_traversal_with_concatenation(numbers, &mut lel, numbers[1], &operators)
}

pub fn tree_traversal_with_concatenation(
    numbers: &Vec<u128>,
    i: &mut usize,
    partial: u128,
    operators: &Vec<fn(u128, u128) -> u128>,
) -> bool {
    if partial > numbers[0] {
        return false;
    } else if *i == numbers.len() {
        return partial == numbers[0];
    } else {
        let mut i_plus = *i;
        let mut i_mul = *i;
        let mut i_concat = *i;
        for (op_counter, operator) in operators.iter().enumerate() {
            let result = operator(partial, numbers[*i]);
            if op_counter == 0 {
                i_plus += 1;
                if tree_traversal_with_concatenation(numbers, &mut i_plus, result, operators) {
                    return true;
                } else {
                    continue;
                }
            } else if op_counter == 1 {
                i_mul += 1;
                if tree_traversal_with_concatenation(numbers, &mut i_mul, result, operators) {
                    return true;
                } else {
                    continue;
                }
            } else {
                i_concat += 1;
                if tree_traversal_with_concatenation(numbers, &mut i_concat, result, operators) {
                    return true;
                } else {
                    continue;
                }
            }
        }
    }
    false
}

fn add(a: u128, b: u128) -> u128 {
    a + b
}
fn multiply(a: u128, b: u128) -> u128 {
    a * b
}
fn concat(num1: u128, num2: u128) -> u128 {
    let mut concat = num1.to_string();
    let concat2 = num2.to_string();
    concat.push_str(&concat2);
    concat.parse::<u128>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1985268524462));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(150077710195188));
    }
}
