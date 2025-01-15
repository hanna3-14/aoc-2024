use itertools::Itertools;
use pathfinding::prelude::{build_path, dijkstra, dijkstra_all};
use std::collections::HashMap;

advent_of_code::solution!(16);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
pub struct Node(usize, usize, char);

pub fn part_one(input: &str) -> Option<u32> {
    let number_of_rows = input.lines().count() - 1;
    let mut map: Vec<Vec<_>> = vec![];

    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let mut start_row = 0;
    let mut end_row = 0;
    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, line) in map.iter().enumerate() {
        if line.contains(&'S') {
            start_row = i;
        }
        if line.contains(&'E') {
            end_row = i;
        }
        let wall_positions: Vec<usize> = map[i].iter().positions(|&char| char == '#').collect();
        walls.entry(i).or_default().extend(wall_positions);
    }

    let start_position = (
        start_row,
        map[start_row].iter().position(|&char| char == 'S').unwrap(),
    );

    let end_position = (
        end_row,
        map[end_row].iter().position(|&char| char == 'E').unwrap(),
    );

    impl Node {
        fn successors(
            &self,
            walls: HashMap<usize, Vec<usize>>,
            number_of_rows: usize,
        ) -> Vec<(Node, usize)> {
            let &Node(row, column, dir) = self;
            let mut possible_moves: Vec<_> = vec![];
            if dir == 'r' && row < number_of_rows && column > 0 && row > 0 {
                if !walls.get(&(row)).unwrap().contains(&(column + 1)) {
                    possible_moves.push((Node(row, column + 1, 'r'), 1));
                }
                if !walls.get(&(row + 1)).unwrap().contains(&(column)) {
                    possible_moves.push((Node(row + 1, column, 'd'), 1001));
                }
                if !walls.get(&(row - 1)).unwrap().contains(&(column)) {
                    possible_moves.push((Node(row - 1, column, 'u'), 1001));
                }
            } else if dir == 'd' && row < number_of_rows && column > 0 && row > 0 {
                if !walls.get(&(row + 1)).unwrap().contains(&(column)) {
                    possible_moves.push((Node(row + 1, column, 'd'), 1));
                }
                if !walls.get(&(row)).unwrap().contains(&(column + 1)) {
                    possible_moves.push((Node(row, column + 1, 'r'), 1001));
                }
                if !walls.get(&(row)).unwrap().contains(&(column - 1)) {
                    possible_moves.push((Node(row, column - 1, 'l'), 1001));
                }
            } else if dir == 'l' && row < number_of_rows && column > 0 && row > 0 {
                if !walls.get(&(row)).unwrap().contains(&(column - 1)) {
                    possible_moves.push((Node(row, column - 1, 'l'), 1));
                }
                if !walls.get(&(row + 1)).unwrap().contains(&(column)) {
                    possible_moves.push((Node(row + 1, column, 'd'), 1001));
                }
                if !walls.get(&(row - 1)).unwrap().contains(&(column)) {
                    possible_moves.push((Node(row - 1, column, 'u'), 1001));
                }
            } else if dir == 'u' && row < number_of_rows && column > 0 && row > 0 {
                if !walls.get(&(row - 1)).unwrap().contains(&(column)) {
                    possible_moves.push((Node(row - 1, column, 'u'), 1));
                }
                if !walls.get(&(row)).unwrap().contains(&(column + 1)) {
                    possible_moves.push((Node(row, column + 1, 'r'), 1001));
                }
                if !walls.get(&(row)).unwrap().contains(&(column - 1)) {
                    possible_moves.push((Node(row, column - 1, 'l'), 1001));
                }
            }
            possible_moves
        }
    }

    let goal1: Node = Node(end_position.0, end_position.1, 'u');
    let goal2: Node = Node(end_position.0, end_position.1, 'r');
    let goal3: Node = Node(end_position.0, end_position.1, 'd');
    let goal4: Node = Node(end_position.0, end_position.1, 'l');
    let result = dijkstra(
        &Node(start_position.0, start_position.1, 'r'),
        |p| p.successors(walls.clone(), number_of_rows),
        |p| *p == goal1 || *p == goal2 || *p == goal3 || *p == goal4,
    );

    Some(result.unwrap().1 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_of_rows = input.lines().count() - 1;
    let mut map: Vec<Vec<_>> = vec![];

    for line in input.lines() {
        map.push(line.chars().collect());
    }

    let mut start_row = 0;
    let mut end_row = 0;
    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, line) in map.iter().enumerate() {
        if line.contains(&'S') {
            start_row = i;
        }
        if line.contains(&'E') {
            end_row = i;
        }
        let wall_positions: Vec<usize> = map[i].iter().positions(|&char| char == '#').collect();
        walls.entry(i).or_default().extend(wall_positions);
    }

    let start_position = (
        start_row,
        map[start_row].iter().position(|&char| char == 'S').unwrap(),
    );

    let end_position = (
        end_row,
        map[end_row].iter().position(|&char| char == 'E').unwrap(),
    );

    let reachables: HashMap<Node, (Node, usize)> =
        dijkstra_all(&Node(start_position.0, start_position.1, 'r'), |p| {
            p.successors(walls.clone(), number_of_rows)
        });

    let goal1: Node = Node(end_position.0, end_position.1, 'u');
    let mut path: Vec<Node> = build_path(&goal1, &reachables);

    let mut total: Vec<(usize, usize)> = vec![];
    for node in path.iter() {
        total.push((node.0, node.1));
    }

    let mut i = 0;
    while i < path.len() {
        let mut goal2: Node = *path.get(i).unwrap();
        let mut other_options: Vec<&(Node, usize)> = vec![];
        goal2.2 = 'r';
        if reachables.contains_key(&goal2) {
            other_options.push(reachables.get(&goal2).unwrap());
        }
        goal2.2 = 'u';
        if reachables.contains_key(&goal2) {
            other_options.push(reachables.get(&goal2).unwrap());
        }
        goal2.2 = 'd';
        if reachables.contains_key(&goal2) {
            other_options.push(reachables.get(&goal2).unwrap());
        }
        goal2.2 = 'l';
        if reachables.contains_key(&goal2) {
            other_options.push(reachables.get(&goal2).unwrap());
        }
        other_options.sort_by(|op1, op2| op1.1.cmp(&op2.1));
        if other_options.len() > 2 && other_options[0].1 + 1000 == other_options[1].1 {
            let mut costs_of_option_0 = 0;
            let mut costs_of_option_1 = 0;
            let following_value = (path.get(i + 1).unwrap().0, path.get(i + 1).unwrap().1);
            let current_value = (path.get(i).unwrap().0, path.get(i).unwrap().1);
            if current_value.0 - following_value.0 == 1 {
                if other_options[0].0 .2 == 'u' {
                    costs_of_option_0 = other_options[0].1 + 1;
                } else {
                    costs_of_option_0 = other_options[0].1 + 1001;
                }
                if other_options[1].0 .2 == 'u' {
                    costs_of_option_1 = other_options[1].1 + 1;
                } else {
                    costs_of_option_1 = other_options[1].1 + 1001;
                }
            } else if following_value.1 - current_value.1 == 1 {
                if other_options[0].0 .2 == 'r' {
                    costs_of_option_0 = other_options[0].1 + 1;
                } else {
                    costs_of_option_0 = other_options[0].1 + 1001;
                }
                if other_options[1].0 .2 == 'r' {
                    costs_of_option_1 = other_options[1].1 + 1;
                } else {
                    costs_of_option_1 = other_options[1].1 + 1001;
                }
            } else if following_value.0 - current_value.0 == 1 {
                if other_options[0].0 .2 == 'd' {
                    costs_of_option_0 = other_options[0].1 + 1;
                } else {
                    costs_of_option_0 = other_options[0].1 + 1001;
                }
                if other_options[1].0 .2 == 'd' {
                    costs_of_option_1 = other_options[1].1 + 1;
                } else {
                    costs_of_option_1 = other_options[1].1 + 1001;
                }
            } else if current_value.1 - following_value.1 == 1 {
                if other_options[0].0 .2 == 'l' {
                    costs_of_option_0 = other_options[0].1 + 1;
                } else {
                    costs_of_option_0 = other_options[0].1 + 1001;
                }
                if other_options[1].0 .2 == 'l' {
                    costs_of_option_1 = other_options[1].1 + 1;
                } else {
                    costs_of_option_1 = other_options[1].1 + 1001;
                }
            }
            if costs_of_option_0 == costs_of_option_1 {
                goal2 = other_options[1].0;
                let path2 = build_path(&goal2, &reachables);
                for node in path2.iter() {
                    if !total.contains(&(node.0, node.1)) {
                        total.push((node.0, node.1));
                    }
                    if !path.contains(node) {
                        path.push(*node);
                    }
                }
            }
        }
        i += 1;
    }

    Some(total.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_example2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_one_example3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(21148));
    }

    #[test]
    fn test_part_one_example4() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(5078));
    }

    #[test]
    fn test_part_one_example5() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(21110));
    }

    #[test]
    fn test_part_one_example6() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(41210));
    }

    #[test]
    fn test_part_one_example7() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(4013));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(105508));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_example2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two_example3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(149));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(548));
    }
}
