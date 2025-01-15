use std::collections::HashMap;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut connection_counter = 0;

    let mut i = lines.len() - 1;

    while i > 0 {
        let computers: Vec<&str> = lines[i].split('-').collect();
        lines.remove(i);
        let lines_with_first_computer: Vec<_> = lines
            .iter()
            .filter(|line| line.contains(computers.first().unwrap()))
            .collect();
        for line in lines_with_first_computer.iter() {
            let first_line_comps: Vec<_> = line.split('-').collect();
            let additional_computer: &str = if first_line_comps
                .first()
                .unwrap()
                .eq(computers.first().unwrap())
            {
                first_line_comps.last().unwrap()
            } else {
                first_line_comps.first().unwrap()
            };
            let lines_with_second_computer: Vec<_> = lines
                .iter()
                .filter(|line| {
                    line.contains(computers.last().unwrap()) && line.contains(additional_computer)
                })
                .collect();
            if lines_with_second_computer.len() == 1
                && (additional_computer.starts_with('t')
                    || computers.first().unwrap().starts_with('t')
                    || computers.last().unwrap().starts_with('t'))
            {
                connection_counter += 1;
            }
        }
        i -= 1;
    }

    Some(connection_counter)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();

    let mut computers_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines.iter() {
        let computers: Vec<&str> = line.split('-').collect();
        computers_map
            .entry(computers.first().unwrap())
            .or_default()
            .push(computers.last().unwrap());
        computers_map
            .entry(computers.last().unwrap())
            .or_default()
            .push(computers.first().unwrap());
    }

    let mut lan_party: Vec<&str> = Vec::new();
    for computer in computers_map.iter() {
        let mut current_lan_party: Vec<&str> = Vec::new();
        let first_computer = computer.0;
        let connected_computers = computer.1;
        current_lan_party.push(first_computer);
        for connected_computer in connected_computers.iter() {
            if pc_is_in_lan_party(connected_computer, &current_lan_party, &computers_map) {
                current_lan_party.push(connected_computer);
            }
        }
        if current_lan_party.len() > lan_party.len() {
            lan_party = current_lan_party;
        }
    }

    lan_party.sort();

    let mut password = String::new();

    for pc in lan_party.iter() {
        password.push_str(pc);
        password.push(',');
    }
    password.pop();

    Some(password)
}

pub fn pc_is_in_lan_party(
    pc: &str,
    lan_party: &[&str],
    computers_map: &HashMap<&str, Vec<&str>>,
) -> bool {
    for comp in lan_party.iter() {
        if !computers_map.get(&pc).unwrap().contains(comp) {
            return false;
        }
    }
    true
}

pub fn find_connection(comp1: &str, comp2: &str, computers_map: &HashMap<&str, Vec<&str>>) -> bool {
    computers_map.get(comp1).unwrap().contains(&comp2)
        || computers_map.get(comp2).unwrap().contains(&comp1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1184));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(
            result,
            Some(String::from("hf,hz,lb,lm,ls,my,ps,qu,ra,uc,vi,xz,yv"))
        );
    }
}
