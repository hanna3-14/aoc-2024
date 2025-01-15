use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut positions_of_trailheads: Vec<(i32, i32)> = vec![];

    let mut map: Vec<Vec<i32>> = vec![];
    for line in lines.iter() {
        let mut numbers: Vec<i32> = vec![];
        for letter in line.chars() {
            numbers.push(letter as i32 - '0' as i32);
        }
        map.push(numbers);
    }

    for (i, line) in lines.iter().enumerate() {
        let columns: Vec<i32> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '0')
            .map(|(i, _)| i as i32)
            .collect::<Vec<i32>>();
        for column in columns.iter() {
            positions_of_trailheads.push((i as i32, *column));
        }
    }

    let mut trail_score: u32 = 0;

    for position in positions_of_trailheads.iter() {
        let mut positions_of_top: Vec<(i32, i32)> = vec![];
        get_trail_score(&map, *position, &mut positions_of_top);
        trail_score += positions_of_top.iter().unique().count() as u32;
    }

    Some(trail_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut positions_of_trailheads: Vec<(i32, i32)> = vec![];

    let mut map: Vec<Vec<i32>> = vec![];
    for line in lines.iter() {
        let mut numbers: Vec<i32> = vec![];
        for letter in line.chars() {
            numbers.push(letter as i32 - '0' as i32);
        }
        map.push(numbers);
    }

    for (i, line) in lines.iter().enumerate() {
        let columns: Vec<i32> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '0')
            .map(|(i, _)| i as i32)
            .collect::<Vec<i32>>();
        for column in columns.iter() {
            positions_of_trailheads.push((i as i32, *column));
        }
    }

    let mut trail_score = 0;

    for position in positions_of_trailheads.iter() {
        find_all_paths(&map, *position, &mut trail_score);
    }

    Some(trail_score as u32)
}

pub fn get_trail_score(
    map: &Vec<Vec<i32>>,
    position: (i32, i32),
    positions_of_top: &mut Vec<(i32, i32)>,
) {
    let number_of_lines = map.len() as i32;
    let number_of_columns = map.first().unwrap().len() as i32;

    if map[position.0 as usize][position.1 as usize] == 9 {
        positions_of_top.push((position.0, position.1));
    } else {
        if position.0 >= 1
            && map[position.0 as usize - 1][position.1 as usize]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            get_trail_score(map, (position.0 - 1, position.1), positions_of_top);
        }
        if position.1 + 1 < number_of_columns
            && map[position.0 as usize][position.1 as usize + 1]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            get_trail_score(map, (position.0, position.1 + 1), positions_of_top);
        }
        if position.0 + 1 < number_of_lines
            && map[position.0 as usize + 1][position.1 as usize]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            get_trail_score(map, (position.0 + 1, position.1), positions_of_top);
        }
        if position.1 >= 1
            && map[position.0 as usize][position.1 as usize - 1]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            get_trail_score(map, (position.0, position.1 - 1), positions_of_top);
        }
    }
}

pub fn find_all_paths(map: &Vec<Vec<i32>>, position: (i32, i32), trail_score: &mut i32) {
    let number_of_lines = map.len() as i32;
    let number_of_columns = map.first().unwrap().len() as i32;
    if map[position.0 as usize][position.1 as usize] == 9 {
        *trail_score += 1;
    } else {
        if position.0 >= 1
            && map[position.0 as usize - 1][position.1 as usize]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            find_all_paths(map, (position.0 - 1, position.1), trail_score);
        }
        if position.1 + 1 < number_of_columns
            && map[position.0 as usize][position.1 as usize + 1]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            find_all_paths(map, (position.0, position.1 + 1), trail_score);
        }
        if position.0 + 1 < number_of_lines
            && map[position.0 as usize + 1][position.1 as usize]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            find_all_paths(map, (position.0 + 1, position.1), trail_score);
        }
        if position.1 >= 1
            && map[position.0 as usize][position.1 as usize - 1]
                == map[position.0 as usize][position.1 as usize] + 1
        {
            find_all_paths(map, (position.0, position.1 - 1), trail_score);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(778));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1925));
    }
}
