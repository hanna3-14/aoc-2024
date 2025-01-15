use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<_>> = vec![];
    let mut instructions: Vec<&str> = vec![];
    let mut is_instruction_section = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            is_instruction_section = true;
            continue;
        }

        if !is_instruction_section {
            map.push(line.chars().collect());
        } else {
            instructions.push(line);
        }
    }

    let mut start_row = 0;
    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut boxes: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, line) in map.iter().enumerate() {
        if line.contains(&'@') {
            start_row = i;
        }
        let wall_positions: Vec<usize> = map[i].iter().positions(|&char| char == '#').collect();
        walls.entry(i).or_default().extend(wall_positions);
        let box_positions: Vec<usize> = map[i].iter().positions(|&char| char == 'O').collect();
        boxes.entry(i).or_default().extend(box_positions);
    }

    let mut start_position = (
        start_row,
        map[start_row].iter().position(|&char| char == '@').unwrap(),
    );

    for line in instructions.iter() {
        for instruction in line.chars() {
            if instruction == '<' {
                start_position.1 -= 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.1 += 1;
                }
                if boxes
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    let mut found_space = false;
                    let mut i = 1;
                    while !found_space {
                        let index_of_box = boxes
                            .get(&start_position.0)
                            .unwrap()
                            .iter()
                            .position(|&number| number == start_position.1)
                            .unwrap();
                        let new_box_position =
                            boxes.entry(start_position.0).or_default()[index_of_box] - i;
                        if walls
                            .entry(start_position.0)
                            .or_default()
                            .contains(&new_box_position)
                        {
                            start_position.1 += 1;
                            break;
                        } else if !boxes
                            .entry(start_position.0)
                            .or_default()
                            .contains(&new_box_position)
                        {
                            boxes.entry(start_position.0).or_default()[index_of_box] -= i;
                            found_space = true;
                        } else {
                            i += 1;
                        }
                    }
                }
            } else if instruction == '^' {
                start_position.0 -= 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.0 += 1;
                }
                if boxes
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    let mut found_space = false;
                    let mut i = 1;
                    while !found_space {
                        let index_to_remove = boxes
                            .entry(start_position.0)
                            .or_default()
                            .iter()
                            .position(|&number| number == start_position.1)
                            .unwrap();
                        let new_box_row = start_position.0 - i;
                        if walls
                            .entry(new_box_row)
                            .or_default()
                            .contains(&start_position.1)
                        {
                            start_position.0 += 1;
                            break;
                        } else if !boxes
                            .entry(new_box_row)
                            .or_default()
                            .contains(&start_position.1)
                        {
                            boxes
                                .entry(start_position.0)
                                .or_default()
                                .remove(index_to_remove);
                            boxes.entry(new_box_row).or_default().push(start_position.1);
                            found_space = true;
                        } else {
                            i += 1;
                        }
                    }
                }
            } else if instruction == '>' {
                start_position.1 += 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.1 -= 1;
                }
                if boxes
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    let mut found_space = false;
                    let mut i = 1;
                    while !found_space {
                        let index_of_box = boxes
                            .get(&start_position.0)
                            .unwrap()
                            .iter()
                            .position(|&number| number == start_position.1)
                            .unwrap();
                        let new_box_position =
                            boxes.entry(start_position.0).or_default()[index_of_box] + i;
                        if walls
                            .entry(start_position.0)
                            .or_default()
                            .contains(&new_box_position)
                        {
                            start_position.1 -= 1;
                            break;
                        } else if !boxes
                            .entry(start_position.0)
                            .or_default()
                            .contains(&new_box_position)
                        {
                            boxes.entry(start_position.0).or_default()[index_of_box] += i;
                            found_space = true;
                        } else {
                            i += 1;
                        }
                    }
                }
            } else if instruction == 'v' {
                start_position.0 += 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.0 -= 1;
                }
                if boxes
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    let mut found_space = false;
                    let mut i = 1;
                    while !found_space {
                        let index_to_remove = boxes
                            .entry(start_position.0)
                            .or_default()
                            .iter()
                            .position(|&number| number == start_position.1)
                            .unwrap();
                        let new_box_row = start_position.0 + i;
                        if walls
                            .entry(new_box_row)
                            .or_default()
                            .contains(&start_position.1)
                        {
                            start_position.0 -= 1;
                            break;
                        } else if !boxes
                            .entry(new_box_row)
                            .or_default()
                            .contains(&start_position.1)
                        {
                            boxes
                                .entry(start_position.0)
                                .or_default()
                                .remove(index_to_remove);
                            boxes.entry(new_box_row).or_default().push(start_position.1);
                            found_space = true;
                        } else {
                            i += 1;
                        }
                    }
                }
            }
        }
    }

    let mut result = 0;

    for row in boxes.iter() {
        for position in 0..row.1.len() {
            result += row.0 * 100 + row.1[position];
        }
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<_>> = vec![];
    let mut instructions: Vec<&str> = vec![];
    let mut is_instruction_section = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            is_instruction_section = true;
            continue;
        }

        if !is_instruction_section {
            let chars: Vec<char> = line.chars().collect();
            let mut row: Vec<char> = vec![];
            for char in chars.iter() {
                if *char == '#' {
                    row.push('#');
                    row.push('#');
                } else if *char == '.' {
                    row.push('.');
                    row.push('.');
                } else if *char == 'O' {
                    row.push('[');
                    row.push(']');
                } else if *char == '@' {
                    row.push('@');
                    row.push('.');
                }
            }
            map.push(row);
        } else {
            instructions.push(line);
        }
    }

    let mut start_row = 0;
    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut boxes: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, line) in map.iter().enumerate() {
        if line.contains(&'@') {
            start_row = i;
        }
        let wall_positions: Vec<usize> = map[i].iter().positions(|&char| char == '#').collect();
        walls.entry(i).or_default().extend(wall_positions);
        let box_positions: Vec<usize> = map[i].iter().positions(|&char| char == '[').collect();
        boxes.entry(i).or_default().extend(box_positions);
    }

    let mut start_position = (
        start_row,
        map[start_row].iter().position(|&char| char == '@').unwrap(),
    );

    for line in instructions.iter() {
        for instruction in line.chars() {
            if instruction == '<' {
                start_position.1 -= 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.1 += 1;
                }
                if boxes
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&(start_position.1 - 1))
                {
                    let mut i: i32 = 0;
                    while boxes
                        .get(&start_position.0)
                        .unwrap()
                        .contains(&(start_position.1 - 1 - i as usize))
                    {
                        i += 2;
                    }
                    i -= 2;
                    // boxes cannot be moved due to a wall
                    let mut move_boxes = true;
                    if walls
                        .get(&start_position.0)
                        .unwrap()
                        .contains(&(start_position.1 - 2 - i as usize))
                    {
                        move_boxes = false;
                    }
                    if move_boxes {
                        // move boxes to the left
                        while i >= 0 {
                            let box_index = boxes
                                .get(&start_position.0)
                                .unwrap()
                                .iter()
                                .position(|&number| number == start_position.1 - 1 - i as usize)
                                .unwrap();
                            boxes.entry(start_position.0).or_default()[box_index] -= 1;
                            i -= 2;
                        }
                    } else {
                        start_position.1 += 1;
                    }
                }
            } else if instruction == '^' {
                start_position.0 -= 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.0 += 1;
                }
                if boxes
                    .get(&(start_position.0))
                    .unwrap()
                    .contains(&(start_position.1 - 1))
                    || boxes
                        .get(&(start_position.0))
                        .unwrap()
                        .contains(&start_position.1)
                {
                    let mut impacted_boxes: Vec<(usize, usize)> = vec![];
                    if boxes
                        .get(&(start_position.0))
                        .unwrap()
                        .contains(&(start_position.1 - 1))
                    {
                        impacted_boxes.push((start_position.0, start_position.1 - 1));
                    } else {
                        impacted_boxes.push((start_position.0, start_position.1));
                    }
                    let mut i = 0;
                    while i < impacted_boxes.len() {
                        if boxes
                            .get(&(impacted_boxes[i].0 - 1))
                            .unwrap()
                            .contains(&impacted_boxes[i].1)
                        {
                            impacted_boxes.push((impacted_boxes[i].0 - 1, impacted_boxes[i].1));
                        }
                        if boxes
                            .get(&(impacted_boxes[i].0 - 1))
                            .unwrap()
                            .contains(&(impacted_boxes[i].1 + 1))
                        {
                            impacted_boxes.push((impacted_boxes[i].0 - 1, impacted_boxes[i].1 + 1));
                        }
                        if boxes
                            .get(&(impacted_boxes[i].0 - 1))
                            .unwrap()
                            .contains(&(impacted_boxes[i].1 - 1))
                        {
                            impacted_boxes.push((impacted_boxes[i].0 - 1, impacted_boxes[i].1 - 1));
                        }
                        i += 1;
                    }
                    let mut move_boxes = true;
                    for boxx in impacted_boxes.iter() {
                        if walls.get(&(boxx.0 - 1)).unwrap().contains(&boxx.1)
                            || walls.get(&(boxx.0 - 1)).unwrap().contains(&(boxx.1 + 1))
                        {
                            move_boxes = false;
                        }
                    }
                    if move_boxes {
                        for boxx in impacted_boxes.iter() {
                            let index_to_remove = boxes
                                .entry(boxx.0)
                                .or_default()
                                .iter()
                                .position(|&number| number == boxx.1)
                                .unwrap();
                            boxes.entry(boxx.0).or_default().remove(index_to_remove);
                            boxes.entry(boxx.0 - 1).or_default().push(boxx.1);
                        }
                    } else {
                        start_position.0 += 1;
                    }
                }
            } else if instruction == '>' {
                start_position.1 += 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.1 -= 1;
                }
                if boxes
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&(start_position.1))
                {
                    let mut i: i32 = 0;
                    while boxes
                        .get(&start_position.0)
                        .unwrap()
                        .contains(&(start_position.1 + i as usize))
                    {
                        i += 2;
                    }
                    i -= 2;
                    // boxes cannot be moved due to a wall
                    let mut move_boxes = true;
                    if walls
                        .get(&start_position.0)
                        .unwrap()
                        .contains(&(start_position.1 + 2 + i as usize))
                    {
                        move_boxes = false;
                    }
                    if move_boxes {
                        // move boxes to the right
                        while i >= 0 {
                            let box_index = boxes
                                .get(&start_position.0)
                                .unwrap()
                                .iter()
                                .position(|&number| number == start_position.1 + i as usize)
                                .unwrap();
                            boxes.entry(start_position.0).or_default()[box_index] += 1;
                            i -= 2;
                        }
                    } else {
                        start_position.1 -= 1;
                    }
                }
            } else if instruction == 'v' {
                start_position.0 += 1;
                if walls
                    .get(&start_position.0)
                    .unwrap()
                    .contains(&start_position.1)
                {
                    start_position.0 -= 1;
                }
                if boxes
                    .get(&(start_position.0))
                    .unwrap()
                    .contains(&(start_position.1 - 1))
                    || boxes
                        .get(&(start_position.0))
                        .unwrap()
                        .contains(&start_position.1)
                {
                    let mut impacted_boxes: Vec<(usize, usize)> = vec![];
                    if boxes
                        .get(&(start_position.0))
                        .unwrap()
                        .contains(&(start_position.1 - 1))
                    {
                        impacted_boxes.push((start_position.0, start_position.1 - 1));
                    } else {
                        impacted_boxes.push((start_position.0, start_position.1));
                    }
                    let mut i = 0;
                    while i < impacted_boxes.len() {
                        if boxes
                            .get(&(impacted_boxes[i].0 + 1))
                            .unwrap()
                            .contains(&impacted_boxes[i].1)
                        {
                            impacted_boxes.push((impacted_boxes[i].0 + 1, impacted_boxes[i].1));
                        }
                        if boxes
                            .get(&(impacted_boxes[i].0 + 1))
                            .unwrap()
                            .contains(&(impacted_boxes[i].1 + 1))
                        {
                            impacted_boxes.push((impacted_boxes[i].0 + 1, impacted_boxes[i].1 + 1));
                        }
                        if boxes
                            .get(&(impacted_boxes[i].0 + 1))
                            .unwrap()
                            .contains(&(impacted_boxes[i].1 - 1))
                        {
                            impacted_boxes.push((impacted_boxes[i].0 + 1, impacted_boxes[i].1 - 1));
                        }
                        i += 1;
                    }
                    let mut move_boxes = true;
                    for boxx in impacted_boxes.iter() {
                        if walls.get(&(boxx.0 + 1)).unwrap().contains(&boxx.1)
                            || walls.get(&(boxx.0 + 1)).unwrap().contains(&(boxx.1 + 1))
                        {
                            move_boxes = false;
                        }
                    }
                    if move_boxes {
                        for boxx in impacted_boxes.iter().unique() {
                            let index_to_remove = boxes
                                .entry(boxx.0)
                                .or_default()
                                .iter()
                                .position(|&number| number == boxx.1)
                                .unwrap();
                            boxes.entry(boxx.0).or_default().remove(index_to_remove);
                            boxes.entry(boxx.0 + 1).or_default().push(boxx.1);
                        }
                    } else {
                        start_position.0 -= 1;
                    }
                }
            }
        }
    }

    let mut result = 0;

    for row in boxes.iter() {
        for position in 0..row.1.len() {
            result += row.0 * 100 + row.1[position];
        }
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_example2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1437174));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_part_two_example2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1437468));
    }
}
