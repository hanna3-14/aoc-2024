use std::collections::HashMap;

use pathfinding::prelude::dijkstra;

advent_of_code::solution!(18);

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
    let bytes: Vec<&str> = input.lines().take(1024).collect();
    let number_of_rows = 71;
    let number_of_columns = 71;

    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();

    for byte in bytes.iter() {
        let coordinate: Vec<u32> = byte
            .split(',')
            .map(|number| number.trim().parse::<u32>().ok().unwrap())
            .collect();
        let x: usize = *coordinate.first().unwrap() as usize;
        let y: usize = *coordinate.last().unwrap() as usize;
        walls.entry(y).or_default().push(x);
    }

    let goal: Node = Node(70, 70);
    let result = dijkstra(
        &Node(0, 0),
        |p| p.successors(walls.clone(), number_of_rows, number_of_columns),
        |p| *p == goal,
    );

    Some(result.unwrap().1 as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut i = 2;
    let mut number_of_additional_rows = 2426 / 2;
    loop {
        let found_path = check_if_path_can_be_found(input, number_of_additional_rows);
        let difference = number_of_additional_rows / i;
        if found_path {
            number_of_additional_rows += difference;
            i *= 2;
        } else {
            number_of_additional_rows -= difference;
        }
        if difference < 1 {
            break;
        }
    }

    Some(
        input
            .lines()
            .nth(1023 + number_of_additional_rows)
            .unwrap()
            .to_string(),
    )
}

pub fn check_if_path_can_be_found(input: &str, i: usize) -> bool {
    let bytes: Vec<&str> = input.lines().take(1024 + i).collect();
    let number_of_rows = 71;
    let number_of_columns = 71;

    let mut walls: HashMap<usize, Vec<usize>> = HashMap::new();

    for byte in bytes.iter() {
        let coordinate: Vec<u32> = byte
            .split(',')
            .map(|number| number.trim().parse::<u32>().ok().unwrap())
            .collect();
        let x: usize = *coordinate.first().unwrap() as usize;
        let y: usize = *coordinate.last().unwrap() as usize;
        walls.entry(y).or_default().push(x);
    }

    let goal: Node = Node(70, 70);
    let result = dijkstra(
        &Node(0, 0),
        |p| p.successors(walls.clone(), number_of_rows, number_of_columns),
        |p| *p == goal,
    );

    result.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(380));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(String::from("26,50")));
    }
}
