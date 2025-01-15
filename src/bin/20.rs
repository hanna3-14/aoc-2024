use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

advent_of_code::solution!(20);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
pub struct Node(usize, usize);

impl Node {
    fn successors(
        &self,
        walls: HashMap<usize, Vec<usize>>,
        number_of_rows: usize,
        number_of_columns: usize,
    ) -> Vec<(Node, usize)> {
        let &Node(y, x) = self;
        let mut possible_moves: Vec<_> = vec![];
        if y > 0 && !walls.get(&(y - 1)).unwrap().contains(&x) {
            possible_moves.push((Node(y - 1, x), 1));
        }
        if y < number_of_rows - 1 && !walls.get(&(y + 1)).unwrap().contains(&x) {
            possible_moves.push((Node(y + 1, x), 1));
        }
        if x > 0 && !walls.get(&y).unwrap().contains(&(x - 1)) {
            possible_moves.push((Node(y, x - 1), 1));
        }
        if x < number_of_columns - 1 && !walls.get(&y).unwrap().contains(&(x + 1)) {
            possible_moves.push((Node(y, x + 1), 1));
        }
        possible_moves
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cheats = 0;
    let lines: Vec<&str> = input.lines().collect();
    let number_of_rows = lines.len();
    let number_of_columns = lines.first()?.len();

    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        if line.chars().contains(&'S') {
            start_position = (i, line.chars().position(|char| char == 'S').unwrap());
        }
        if line.chars().contains(&'E') {
            end_position = (i, line.chars().position(|char| char == 'E').unwrap());
        }
        let wall_positions: Vec<usize> = line.chars().positions(|char| char == '#').collect();
        walls.entry(i).or_default().extend(wall_positions);
    }

    let goal: Node = Node(end_position.0, end_position.1);
    let original_path = dijkstra(
        &Node(start_position.0, start_position.1),
        |p| p.successors(walls.clone(), number_of_rows, number_of_columns),
        |p| *p == goal,
    );

    for node in original_path.as_ref().unwrap().0.iter() {
        let mut nodes: Vec<Node> = vec![];
        if node.1 + 2 < number_of_columns
            && !walls.entry(node.0).or_default().contains(&(node.1 + 2))
            && walls.entry(node.0).or_default().contains(&(node.1 + 1))
        {
            nodes.push(Node(node.0, node.1 + 2));
            if original_path
                .as_ref()
                .unwrap()
                .0
                .contains(&Node(node.0, node.1 + 2))
                && original_path
                    .as_ref()
                    .unwrap()
                    .0
                    .iter()
                    .position(|n| *n == Node(node.0, node.1 + 2))
                    .unwrap() as i32
                    - original_path
                        .as_ref()
                        .unwrap()
                        .0
                        .iter()
                        .position(|n| n == node)
                        .unwrap() as i32
                    - 2
                    >= 100
            {
                cheats += 1;
            }
        }
        if node.0 + 2 < number_of_rows
            && !walls.entry(node.0 + 2).or_default().contains(&node.1)
            && walls.entry(node.0 + 1).or_default().contains(&node.1)
        {
            nodes.push(Node(node.0 + 2, node.1));
            if original_path
                .as_ref()
                .unwrap()
                .0
                .contains(&Node(node.0 + 2, node.1))
                && original_path
                    .as_ref()
                    .unwrap()
                    .0
                    .iter()
                    .position(|n| *n == Node(node.0 + 2, node.1))
                    .unwrap() as i32
                    - original_path
                        .as_ref()
                        .unwrap()
                        .0
                        .iter()
                        .position(|n| n == node)
                        .unwrap() as i32
                    - 2
                    >= 100
            {
                cheats += 1;
            }
        }
        if node.1 as i32 - 2 > 0
            && !walls.entry(node.0).or_default().contains(&(node.1 - 2))
            && walls.entry(node.0).or_default().contains(&(node.1 - 1))
        {
            nodes.push(Node(node.0, node.1 - 2));
            if original_path
                .as_ref()
                .unwrap()
                .0
                .contains(&Node(node.0, node.1 - 2))
                && original_path
                    .as_ref()
                    .unwrap()
                    .0
                    .iter()
                    .position(|n| *n == Node(node.0, node.1 - 2))
                    .unwrap() as i32
                    - original_path
                        .as_ref()
                        .unwrap()
                        .0
                        .iter()
                        .position(|n| n == node)
                        .unwrap() as i32
                    - 2
                    >= 100
            {
                cheats += 1;
            }
        }
        if node.0 as i32 - 2 > 0
            && !walls.entry(node.0 - 2).or_default().contains(&(node.1))
            && walls.entry(node.0 - 1).or_default().contains(&node.1)
        {
            nodes.push(Node(node.0 - 2, node.1));
            if original_path
                .as_ref()
                .unwrap()
                .0
                .contains(&Node(node.0 - 2, node.1))
                && original_path
                    .as_ref()
                    .unwrap()
                    .0
                    .iter()
                    .position(|n| *n == Node(node.0 - 2, node.1))
                    .unwrap() as i32
                    - original_path
                        .as_ref()
                        .unwrap()
                        .0
                        .iter()
                        .position(|n| n == node)
                        .unwrap() as i32
                    - 2
                    >= 100
            {
                cheats += 1;
            }
        }
    }

    Some(cheats)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cheats = 0;
    let lines: Vec<&str> = input.lines().collect();
    let number_of_rows = lines.len();
    let number_of_columns = lines.first()?.len();

    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        if line.chars().contains(&'S') {
            start_position = (i, line.chars().position(|char| char == 'S').unwrap());
        }
        if line.chars().contains(&'E') {
            end_position = (i, line.chars().position(|char| char == 'E').unwrap());
        }
        let wall_positions: Vec<usize> = line.chars().positions(|char| char == '#').collect();
        walls.entry(i).or_default().extend(wall_positions);
    }

    let goal: Node = Node(end_position.0, end_position.1);
    let dijkstra_path = dijkstra(
        &Node(start_position.0, start_position.1),
        |p| p.successors(walls.clone(), number_of_rows, number_of_columns),
        |p| *p == goal,
    );

    let original_time = dijkstra_path.as_ref().unwrap().1 as i32;
    let original_path = dijkstra_path.clone().unwrap().0;

    for node in original_path.iter() {
        let manhattan_fields =
            get_manhattan_fields((node.0, node.1), number_of_rows, number_of_columns, &walls);
        for entry in manhattan_fields.iter() {
            for position in entry.1.iter().unique() {
                let end = (*entry.0, *position);
                let time_until_cheat = original_path
                    .iter()
                    .position(|n| n.0 == node.0 && n.1 == node.1)
                    .unwrap();
                let time_after_cheat = original_time as i32
                    - original_path
                        .iter()
                        .position(|n| n.0 == end.0 && n.1 == end.1)
                        .unwrap() as i32;
                let cheat_time = get_manhattan_distance((node.0, node.1), end) as usize;
                let cheated_time = time_until_cheat as i32 + cheat_time as i32 + time_after_cheat;
                let time_saving = original_time - cheated_time as i32;
                if time_saving >= 100 {
                    cheats += 1;
                }
            }
        }
    }

    Some(cheats)
}

pub fn get_manhattan_fields(
    center: (usize, usize),
    number_of_rows: usize,
    number_of_columns: usize,
    walls: &HashMap<usize, Vec<usize>>,
) -> HashMap<usize, Vec<usize>> {
    let mut manhattan_fields: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..21 {
        for j in 0..(21 - i) {
            if center.0 + i >= number_of_rows || center.1 + j >= number_of_columns {
                break;
            } else if !walls
                .get(&(center.0 + i))
                .unwrap()
                .contains(&(center.1 + j))
            {
                manhattan_fields
                    .entry(center.0 + i)
                    .or_default()
                    .push(center.1 + j);
            }
        }
    }
    for i in 0..21 {
        for j in 0..(21 - i) {
            if center.0 + i >= number_of_rows || center.1 as i32 - j as i32 <= 0 {
                break;
            } else if !walls
                .get(&(center.0 + i))
                .unwrap()
                .contains(&(center.1 - j))
            {
                manhattan_fields
                    .entry(center.0 + i)
                    .or_default()
                    .push(center.1 - j);
            }
        }
    }
    for i in 1..21 {
        for j in 0..(21 - i) {
            if center.0 as i32 - i as i32 <= 0 || center.1 + j >= number_of_columns {
                break;
            } else if !walls
                .get(&(center.0 - i))
                .unwrap()
                .contains(&(center.1 + j))
            {
                manhattan_fields
                    .entry(center.0 - i)
                    .or_default()
                    .push(center.1 + j);
            }
        }
    }
    for i in 1..21 {
        for j in 0..(21 - i) {
            if center.0 as i32 - i as i32 <= 0 || center.1 as i32 - j as i32 <= 0 {
                break;
            } else if !walls
                .get(&(center.0 - i))
                .unwrap()
                .contains(&(center.1 - j))
            {
                manhattan_fields
                    .entry(center.0 - i)
                    .or_default()
                    .push(center.1 - j);
            }
        }
    }

    manhattan_fields
}

pub fn get_manhattan_distance(start: (usize, usize), end: (usize, usize)) -> i32 {
    (start.0 as i32 - end.0 as i32).abs() + (start.1 as i32 - end.1 as i32).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1448));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1017615));
    }
}
