use regex::Regex;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").ok()?;

    let number_of_rows = 103;
    let number_of_columns = 101;

    let robots: Vec<_> = input
        .lines()
        .map(|line| {
            let position: Vec<(i32, i32)> = re
                .captures_iter(line)
                .filter_map(|caps| {
                    Some((
                        caps.get(1)?.as_str().parse::<i32>().ok().unwrap(),
                        caps.get(2)?.as_str().parse::<i32>().ok().unwrap(),
                    ))
                })
                .collect();
            let velocities: Vec<(i32, i32)> = re
                .captures_iter(line)
                .filter_map(|caps| {
                    Some((
                        caps.get(3)?.as_str().parse::<i32>().ok().unwrap(),
                        caps.get(4)?.as_str().parse::<i32>().ok().unwrap(),
                    ))
                })
                .collect();
            [*position.first().unwrap(), *velocities.first().unwrap()].to_vec()
        })
        .collect();

    let mut quadrant1 = 0;
    let mut quadrant2 = 0;
    let mut quadrant3 = 0;
    let mut quadrant4 = 0;

    for robot in robots.iter() {
        let robot_position = (
            (robot.first().unwrap().0 + 100 * robot.get(1).unwrap().0)
                .rem_euclid(number_of_columns),
            (robot.first().unwrap().1 + 100 * robot.get(1).unwrap().1).rem_euclid(number_of_rows),
        );
        if robot_position.0 < number_of_columns / 2 && robot_position.1 < number_of_rows / 2 {
            quadrant1 += 1;
        } else if robot_position.0 > number_of_columns / 2 && robot_position.1 < number_of_rows / 2
        {
            quadrant2 += 1;
        } else if robot_position.0 < number_of_columns / 2 && robot_position.1 > number_of_rows / 2
        {
            quadrant3 += 1;
        } else if robot_position.0 > number_of_columns / 2 && robot_position.1 > number_of_rows / 2
        {
            quadrant4 += 1;
        }
    }

    Some(quadrant1 * quadrant2 * quadrant3 * quadrant4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").ok()?;

    let number_of_rows = 103;
    let number_of_columns = 101;

    let robots: Vec<_> = input
        .lines()
        .map(|line| {
            let position: Vec<(i32, i32)> = re
                .captures_iter(line)
                .filter_map(|caps| {
                    Some((
                        caps.get(1)?.as_str().parse::<i32>().ok().unwrap(),
                        caps.get(2)?.as_str().parse::<i32>().ok().unwrap(),
                    ))
                })
                .collect();
            let velocities: Vec<(i32, i32)> = re
                .captures_iter(line)
                .filter_map(|caps| {
                    Some((
                        caps.get(3)?.as_str().parse::<i32>().ok().unwrap(),
                        caps.get(4)?.as_str().parse::<i32>().ok().unwrap(),
                    ))
                })
                .collect();
            [*position.first().unwrap(), *velocities.first().unwrap()].to_vec()
        })
        .collect();

    let mut min_sf = u32::MAX;
    let mut best_iteration: u32 = 0;

    for i in 0..number_of_rows * number_of_columns {
        let mut quadrant1 = 0;
        let mut quadrant2 = 0;
        let mut quadrant3 = 0;
        let mut quadrant4 = 0;
        for robot in robots.iter() {
            let robot_position = (
                (robot.first().unwrap().0 + i * robot.get(1).unwrap().0)
                    .rem_euclid(number_of_columns),
                (robot.first().unwrap().1 + i * robot.get(1).unwrap().1).rem_euclid(number_of_rows),
            );
            if robot_position.0 < number_of_columns / 2 && robot_position.1 < number_of_rows / 2 {
                quadrant1 += 1;
            } else if robot_position.0 > number_of_columns / 2
                && robot_position.1 < number_of_rows / 2
            {
                quadrant2 += 1;
            } else if robot_position.0 < number_of_columns / 2
                && robot_position.1 > number_of_rows / 2
            {
                quadrant3 += 1;
            } else if robot_position.0 > number_of_columns / 2
                && robot_position.1 > number_of_rows / 2
            {
                quadrant4 += 1;
            }
        }
        let sf = quadrant1 * quadrant2 * quadrant3 * quadrant4;
        if sf < min_sf {
            min_sf = sf;
            best_iteration = i as u32;
        }
    }

    Some(best_iteration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(225521010));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7774));
    }
}
