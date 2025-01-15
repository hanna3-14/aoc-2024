use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited_positions: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();
    let number_of_rows = lines.len();
    let number_of_columns = lines.first()?.len();

    let mut map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect();

    let mut line_number = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains("^"))
        .map(|(i, _)| i)
        .unwrap();
    let mut column_number = lines[line_number]
        .chars()
        .enumerate()
        .find(|(_, col)| col.eq(&'^'))
        .map(|(i, _)| i)
        .unwrap();

    let mut direction = "up";

    // set the start position as visited
    map[line_number][column_number] = 'X';

    while line_number > 0
        && line_number < number_of_rows - 1
        && column_number > 0
        && column_number < number_of_columns - 1
    {
        if direction.eq("up") {
            line_number -= 1;
            if map[line_number][column_number] == '#' {
                direction = "right";
                line_number += 1;
            } else {
                map[line_number][column_number] = 'X';
            }
        } else if direction.eq("right") {
            column_number += 1;
            if map[line_number][column_number] == '#' {
                direction = "down";
                column_number -= 1;
            } else {
                map[line_number][column_number] = 'X';
            }
        } else if direction.eq("down") {
            line_number += 1;
            if map[line_number][column_number] == '#' {
                direction = "left";
                line_number -= 1;
            } else {
                map[line_number][column_number] = 'X';
            }
        } else if direction.eq("left") {
            column_number -= 1;
            if map[line_number][column_number] == '#' {
                direction = "up";
                column_number += 1;
            } else {
                map[line_number][column_number] = 'X';
            }
        }
    }

    for line in map {
        visited_positions += line.iter().filter(|&&char| char == 'X').count() as u32;
    }

    Some(visited_positions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut loops: u32 = 0;

    let lines: Vec<&str> = input.lines().collect();
    let number_of_rows = lines.len();
    let number_of_columns = lines.first()?.len();

    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect();

    let line_number = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains("^"))
        .map(|(i, _)| i)
        .unwrap();
    let column_number = lines[line_number]
        .chars()
        .enumerate()
        .find(|(_, col)| col.eq(&'^'))
        .map(|(i, _)| i)
        .unwrap();

    let start_position = (line_number, column_number);

    for i in 0..number_of_rows {
        let loops_per_row = map[i]
            .par_iter()
            .enumerate()
            .filter(|(j, &c)| {
                // if an obstacle is already placed there or if it is the starting point, it should be skipped
                if c == '#' || c == '^' {
                    return false;
                }
                let mut local_map = map.clone();
                local_map[i][*j] = '#';
                // reset all parameters
                let mut line_number = start_position.0;
                let mut column_number = start_position.1;
                let mut obstacles = Vec::new();
                let mut repeated_obsticals = 0;
                let mut direction = "up";
                let mut last_time_i_met_this_obsticale = 100000000;
                while line_number > 0
                    && line_number < number_of_rows - 1
                    && column_number > 0
                    && column_number < number_of_columns - 1
                {
                    if direction.eq("up") {
                        line_number -= 1;
                        if local_map[line_number][column_number] == '#' {
                            direction = "right";
                            if obstacles.contains(&(line_number, column_number)) {
                                last_time_i_met_this_obsticale = obstacles.len()
                                    - obstacles
                                        .iter()
                                        .enumerate()
                                        .find(|&(_, (line, column))| {
                                            line == &line_number && column == &column_number
                                        })
                                        .map(|(i, _)| i)
                                        .unwrap();
                                repeated_obsticals += 1;
                            } else {
                                repeated_obsticals = 0;
                            }
                            obstacles.push((line_number, column_number));
                            line_number += 1;
                        }
                    } else if direction.eq("right") {
                        column_number += 1;
                        if local_map[line_number][column_number] == '#' {
                            direction = "down";
                            if obstacles.contains(&(line_number, column_number)) {
                                last_time_i_met_this_obsticale = obstacles.len()
                                    - obstacles
                                        .iter()
                                        .enumerate()
                                        .find(|&(_, (line, column))| {
                                            line == &line_number && column == &column_number
                                        })
                                        .map(|(i, _)| i)
                                        .unwrap();
                                repeated_obsticals += 1;
                            } else {
                                repeated_obsticals = 0;
                            }
                            obstacles.push((line_number, column_number));
                            column_number -= 1;
                        }
                    } else if direction.eq("down") {
                        line_number += 1;
                        if local_map[line_number][column_number] == '#' {
                            direction = "left";
                            if obstacles.contains(&(line_number, column_number)) {
                                last_time_i_met_this_obsticale = obstacles.len()
                                    - obstacles
                                        .iter()
                                        .enumerate()
                                        .find(|&(_, (line, column))| {
                                            line == &line_number && column == &column_number
                                        })
                                        .map(|(i, _)| i)
                                        .unwrap();
                                repeated_obsticals += 1;
                            } else {
                                repeated_obsticals = 0;
                            }
                            obstacles.push((line_number, column_number));
                            line_number -= 1;
                        }
                    } else if direction.eq("left") {
                        column_number -= 1;
                        if local_map[line_number][column_number] == '#' {
                            direction = "up";
                            if obstacles.contains(&(line_number, column_number)) {
                                last_time_i_met_this_obsticale = obstacles.len()
                                    - obstacles
                                        .iter()
                                        .enumerate()
                                        .find(|&(_, (line, column))| {
                                            line == &line_number && column == &column_number
                                        })
                                        .map(|(i, _)| i)
                                        .unwrap();
                                repeated_obsticals += 1;
                            } else {
                                repeated_obsticals = 0;
                            }
                            obstacles.push((line_number, column_number));
                            column_number += 1;
                        }
                    }
                    if repeated_obsticals >= last_time_i_met_this_obsticale {
                        local_map[i][*j] = '.';
                        return true;
                    }
                }
                local_map[i][*j] = '.';
                false
            })
            .count();

        loops += loops_per_row as u32;
    }

    Some(loops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5551));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1939));
    }
}
