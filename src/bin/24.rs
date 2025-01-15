use std::collections::HashMap;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let mut is_instructions_section = false;

    let mut registers: HashMap<&str, u8> = HashMap::new();
    let mut instructions: Vec<Vec<&str>> = Vec::new();

    for line in lines.iter() {
        if line.trim().is_empty() {
            is_instructions_section = true;
            continue;
        }

        if !is_instructions_section {
            let splits: Vec<_> = line.split(':').collect();
            registers.insert(
                splits.first().unwrap(),
                splits.last().unwrap().trim().parse::<u8>().unwrap(),
            );
        } else {
            let splits: Vec<_> = line.split_whitespace().collect();
            instructions.push(splits)
        }
    }

    let mut i = 0;

    while !instructions.is_empty() {
        let reg1 = instructions.get(i).unwrap().first().unwrap();
        let reg2 = instructions.get(i).unwrap().get(2).unwrap();
        if registers.contains_key(reg1) && registers.contains_key(reg2) {
            let op1 = registers.get(reg1).unwrap();
            let op2 = registers.get(reg2).unwrap();
            let operand = *instructions.get(i).unwrap().get(1).unwrap();
            match operand {
                "OR" => {
                    registers.insert(
                        instructions.get(i).unwrap().get(4).unwrap(),
                        or_operation(op1, op2),
                    );
                }
                "XOR" => {
                    registers.insert(
                        instructions.get(i).unwrap().get(4).unwrap(),
                        xor_operation(op1, op2),
                    );
                }
                "AND" => {
                    registers.insert(
                        instructions.get(i).unwrap().get(4).unwrap(),
                        and_operation(op1, op2),
                    );
                }
                _ => println!("invalid operation"),
            }
            instructions.remove(i);
        }
        i += 1;
        if i >= instructions.len() {
            i = 0;
        }
    }

    let filtered_register: HashMap<_, _> = registers
        .iter()
        .filter(|reg| reg.0.starts_with('z'))
        .collect();

    let mut exponent = 1;

    let mut result = 0;

    for i in 0..filtered_register.len() {
        if i < 10 {
            let value: Vec<_> = filtered_register
                .iter()
                .filter(|&(k, _)| k.contains(&("z0".to_owned() + i.to_string().as_str())))
                .map(|(_, v)| *v)
                .collect();
            if **value.first().unwrap() == 1 {
                result += exponent;
            }
            exponent *= 2;
        } else {
            let value: Vec<_> = filtered_register
                .iter()
                .filter(|&(k, _)| k.contains(&("z".to_owned() + i.to_string().as_str())))
                .map(|(_, v)| *v)
                .collect();
            if **value.first().unwrap() == 1 {
                result += exponent;
            }
            exponent *= 2;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let mut is_instructions_section = false;

    let mut instructions: Vec<Vec<&str>> = Vec::new();

    for line in lines.iter() {
        if line.trim().is_empty() {
            is_instructions_section = true;
            continue;
        }

        if !is_instructions_section {
        } else {
            let splits: Vec<_> = line.split_whitespace().collect();
            instructions.push(splits)
        }
    }

    let mut registers_to_swap: Vec<&str> = Vec::new();

    for instruction in instructions.iter() {
        if (instruction.last().unwrap().starts_with('z')
            && !instruction.last().unwrap().eq(&"z45")
            && !instruction.get(1).unwrap().eq(&"XOR"))
            || (!instruction.last().unwrap().starts_with('z')
                && !instruction.first().unwrap().starts_with('x')
                && !instruction.first().unwrap().starts_with('y')
                && instruction.get(1).unwrap().eq(&"XOR"))
        {
            registers_to_swap.push(instruction.last().unwrap());
        } else if (instruction.first().unwrap().starts_with('x')
            || instruction.first().unwrap().starts_with('y'))
            && instruction.get(1).unwrap().eq(&"XOR")
            && !instruction.last().unwrap().eq(&"z00")
        {
            let other_xor_gate: Vec<_> = instructions
                .iter()
                .filter(|ins| {
                    ins.get(1).unwrap().eq(&"XOR")
                        && (ins.first().unwrap().eq(instruction.last().unwrap())
                            || ins.get(2).unwrap().eq(instruction.last().unwrap()))
                })
                .collect();
            if other_xor_gate.is_empty() {
                registers_to_swap.push(instruction.last().unwrap());
            }
        } else if (instruction.first().unwrap().starts_with('x')
            || instruction.first().unwrap().starts_with('y'))
            && instruction.get(1).unwrap().eq(&"AND")
            && !instruction.first().unwrap().contains("00")
        {
            let or_gates: Vec<_> = instructions
                .iter()
                .filter(|ins| {
                    ins.get(1).unwrap().eq(&"OR")
                        && (ins.first().unwrap().eq(instruction.last().unwrap())
                            || ins.get(2).unwrap().eq(instruction.last().unwrap()))
                })
                .collect();
            if or_gates.is_empty() {
                registers_to_swap.push(instruction.last().unwrap());
            }
        }
    }

    let mut registers_string = String::new();
    registers_to_swap.sort();

    for register in registers_to_swap.iter() {
        registers_string.push_str(register);
        registers_string.push(',');
    }
    registers_string.pop();

    Some(registers_string)
}

pub fn or_operation(op1: &u8, op2: &u8) -> u8 {
    if *op1 == 1 || *op2 == 1 {
        return 1;
    }
    0
}

pub fn xor_operation(op1: &u8, op2: &u8) -> u8 {
    if *op1 == 1 && *op2 == 0 || *op1 == 0 && *op2 == 1 {
        return 1;
    }
    0
}

pub fn and_operation(op1: &u8, op2: &u8) -> u8 {
    if *op1 == 1 && *op2 == 1 {
        return 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_example2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(53325321422566));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(
            result,
            Some(String::from("fkb,nnr,rdn,rqf,rrn,z16,z31,z37"))
        );
    }
}
