advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut register_a = input
        .lines()
        .find(|line| line.starts_with("Register A"))
        .and_then(|line| line.split(':').nth(1)?.trim().parse::<u32>().ok())
        .unwrap();
    let mut register_b = input
        .lines()
        .find(|line| line.starts_with("Register B"))
        .and_then(|line| line.split(':').nth(1)?.trim().parse::<u32>().ok())
        .unwrap();
    let mut register_c = input
        .lines()
        .find(|line| line.starts_with("Register C"))
        .and_then(|line| line.split(':').nth(1)?.trim().parse::<u32>().ok())
        .unwrap();
    let program_input: Vec<_> = input
        .lines()
        .find(|line| line.starts_with("Program"))
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap()
        .split(',')
        .collect();
    let mut program: Vec<u32> = vec![];

    for number in program_input.iter() {
        program.push(number.parse().ok().unwrap());
    }

    let mut instruction_pointer = 0;

    let mut result = String::new();

    loop {
        let mut increase_instruction_pointer = true;
        let opcode = program[instruction_pointer];
        let combo_operand: u32;
        if program[instruction_pointer + 1] == 4 {
            combo_operand = register_a;
        } else if program[instruction_pointer + 1] == 5 {
            combo_operand = register_b;
        } else if program[instruction_pointer + 1] == 6 {
            combo_operand = register_c;
        } else {
            combo_operand = program[instruction_pointer + 1];
        }
        let literal_operand = program[instruction_pointer + 1];

        if opcode == 0 {
            let base: i32 = 2;
            let denominator = base.pow(combo_operand) as u32;
            register_a /= denominator;
        } else if opcode == 1 {
            register_b ^= literal_operand;
        } else if opcode == 2 {
            register_b = combo_operand % 8;
        } else if opcode == 3 {
            if register_a != 0 {
                instruction_pointer = literal_operand as usize;
                increase_instruction_pointer = false;
            }
        } else if opcode == 4 {
            register_b ^= register_c;
        } else if opcode == 5 {
            result += &(combo_operand % 8).to_string();
            result += ",";
        } else if opcode == 6 {
            let base: i32 = 2;
            let denominator = base.pow(combo_operand) as u32;
            register_b = register_a / denominator;
        } else if opcode == 7 {
            let base: i32 = 2;
            let denominator = base.pow(combo_operand) as u32;
            register_c = register_a / denominator;
        }

        if increase_instruction_pointer {
            instruction_pointer += 2;
            if instruction_pointer >= program.len() {
                break;
            }
        }
    }
    result.pop();

    Some(result.clone())
}

pub fn part_two(input: &str) -> Option<u64> {
    let program_input: Vec<_> = input
        .lines()
        .find(|line| line.starts_with("Program"))
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap()
        .split(',')
        .collect();
    let mut program: Vec<u64> = vec![];

    for number in program_input.iter() {
        program.push(number.parse().ok().unwrap());
    }

    let mut a: u64 = 0;
    for instruction in program.iter().rev() {
        a *= 8;
        for i in 0..10 {
            let print_value = calculate_print_value(a + i);
            if print_value == *instruction {
                a += i;
                break;
            }
        }
    }

    Some(a)
}

pub fn calculate_print_value(a: u64) -> u64 {
    let mut b = a % 8;
    if b % 2 == 0 {
        b += 1;
    } else {
        b -= 1;
    }
    let base: i32 = 2;
    let denominator = base.pow(b as u32) as u64;
    let c = a / denominator;
    b ^= c;
    b ^= 4;
    b % 8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(String::from("4,6,1,4,2,1,3,1,6")));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(202366627359274));
    }
}
