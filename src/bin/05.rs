use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    let (rules, updates) = read_input(input);

    // check if update is valid
    for update in updates.iter() {
        let len_of_update = update.len();
        let mut counter_of_valid_pages = 0;
        for (j, page) in update.iter().enumerate().take(len_of_update - 1) {
            if rules.contains_key(page) && rules.get(page).unwrap().contains(&update[j + 1]) {
                counter_of_valid_pages += 1;
                continue;
            }
        }
        if len_of_update - 1 == counter_of_valid_pages {
            sum += update[len_of_update / 2] as u32;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    let (rules, mut updates) = read_input(input);

    // check if update is valid
    for update in updates.iter_mut() {
        let len_of_update = update.len();
        let mut counter_of_valid_pages = 0;
        for (j, page) in update.iter().enumerate().take(len_of_update - 1) {
            if rules.contains_key(page) && rules.get(page).unwrap().contains(&update[j + 1]) {
                counter_of_valid_pages += 1;
                continue;
            }
        }
        // rearrange update if it is not valid
        if len_of_update - 1 != counter_of_valid_pages {
            let mut page_ranking = vec![];
            for page in update.iter().take(len_of_update) {
                let page_rule = if rules.contains_key(page) {
                    rules.get(page).unwrap()
                } else {
                    &vec![]
                }; // contains all numbers that are allowed to appear after this page
                let matches = update
                    .iter()
                    .filter(|number| page_rule.contains(number))
                    .collect::<Vec<&i32>>()
                    .len(); // counts how many of the required numbers are allowed to appear after this page
                page_ranking.push((page, matches)); // matches each page with the amount of allowed numbers after it
            }
            page_ranking.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            sum += *page_ranking[page_ranking.len() / 2].0 as u32;
        }
    }

    Some(sum)
}

pub fn read_input(input: &str) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];
    let mut is_update_section = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            is_update_section = true;
            continue;
        }

        if !is_update_section {
            let numbers: Vec<i32> = line.split('|').map(|n| n.parse::<i32>().unwrap()).collect();
            rules
                .entry(numbers[0])
                .or_insert([numbers[1]].to_vec())
                .push(numbers[1]);
        } else {
            let numbers: Vec<i32> = line
                .split(',')
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect();
            updates.push(numbers);
        }
    }
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4637));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6370));
    }
}
