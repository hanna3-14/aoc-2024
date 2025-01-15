advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = input.lines().next().unwrap();

    // -1 represents empty space, the numbers > 0 represent the file part ids
    let mut disk_layout: Vec<i64> = vec![];

    let mut is_file = true;

    let mut id_counter = 0;

    for number in disk_map.chars() {
        if is_file {
            for _ in 0..number as i32 - '0' as i32 {
                disk_layout.push(id_counter);
            }
            id_counter += 1;
            is_file = false;
        } else {
            for _ in 0..number as i32 - '0' as i32 {
                disk_layout.push(-1);
            }
            is_file = true;
        }
    }

    let mut checksum: i64 = 0;
    let mut i = 0;
    while i < disk_layout.len() {
        if disk_layout[i] == -1 {
            checksum += disk_layout.last().unwrap() * i as i64;
            disk_layout.pop();
            while *disk_layout.last().unwrap() == -1 {
                disk_layout.pop();
            }
        } else {
            checksum += disk_layout[i] * i as i64;
        }
        i += 1;
    }

    Some(checksum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = input.lines().next().unwrap();

    // -1 represents empty space, the numbers > 0 represent the file part ids
    let mut grouped_disk_layout: Vec<(i64, i64)> = Vec::new();

    let mut is_file = true;
    let mut id_counter = 0;

    for number in disk_map.chars() {
        if is_file {
            grouped_disk_layout.push((id_counter, number as i64 - '0' as i64));
            id_counter += 1;
            is_file = false;
        } else {
            grouped_disk_layout.push((-1, number as i64 - '0' as i64));
            is_file = true;
        }
    }

    let mut checksum: i64 = 0;
    let mut counter = 0;

    let mut i = 0;
    while i < grouped_disk_layout.len() {
        // move files to front and add them to the checksum
        if grouped_disk_layout[i].0 == -1 {
            let (file, file_index) =
                find_last_file_that_fits(&grouped_disk_layout, grouped_disk_layout[i].1);
            if file_index > i {
                // check again if another file can fit in the same space
                grouped_disk_layout[i].1 -= grouped_disk_layout[file_index].1;
                i -= 1;
                // remove file from the end of the disk
                grouped_disk_layout[file_index].0 = 0;
                for _ in 0..file.1 {
                    if file.0 != -1 {
                        checksum += file.0 * counter as i64;
                    }
                    counter += 1;
                }
                // no file fits in anymore
            } else {
                for _ in 0..grouped_disk_layout[i].1 {
                    counter += 1;
                }
            }
            // add files that cannot be moved
        } else {
            for _ in 0..grouped_disk_layout[i].1 {
                if grouped_disk_layout[i].0 != -1 {
                    checksum += grouped_disk_layout[i].0 * counter as i64;
                }
                counter += 1;
            }
        }
        i += 1;
    }

    Some(checksum as u64)
}

pub fn find_last_file_that_fits(layout: &[(i64, i64)], space: i64) -> ((i64, i64), usize) {
    for i in (0..layout.len()).rev() {
        if layout[i].0 != -1 && layout[i].0 != 0 && layout[i].1 <= space {
            return (layout[i], i);
        }
    }
    (layout[0], 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6382875730645));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6420913943576));
    }
}
