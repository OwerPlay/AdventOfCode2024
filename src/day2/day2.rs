use crate::utils::utils;

pub fn day2part1() -> String {
    let lines = utils::read_lines("input/day2.txt");
    let divided: Vec<Vec<u32>> = lines.iter().map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect()).collect();
    let mut counter = 0;
    divided.iter().for_each(|line | {
        if test_line(line) {
            counter += 1;
        }
    });
    counter.to_string()
}

pub fn day2part2() -> String {
    let lines = utils::read_lines("input/day2.txt");
    let divided: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    let mut counter = 0;

    divided.iter().for_each(|line| {
        if test_line(line) {
            counter += 1;
        } else {
            let mut is_ok = false;
            for i in 0..line.len() {
                let mut modified_line = line.clone();
                modified_line.remove(i);
                if test_line(&modified_line) {
                    is_ok = true;
                    break;
                }
            }
            if is_ok {
                counter += 1;
            }
        }
    });

    counter.to_string()
}
fn test_line(line: &Vec<u32>) -> bool {
    let mut sorted_line = line.clone();
    let mut sorted = false;
    sorted_line.sort_unstable();
    if sorted_line == *line {
        sorted = true;
    } else {
        let mut sorted_line = line.clone();
        sorted_line.sort_unstable_by(|a, b| b.cmp(a));
        if sorted_line == *line {
            sorted = true;
        }
    }

    let mut is_ok = sorted;
    if sorted {
        let mut previous = 0;
        line.iter().for_each(|num| {
            if previous != 0 {
                if num.abs_diff(previous) == 0 || num.abs_diff(previous) > 3 {
                    is_ok = false;
                }
            }
            previous = *num;
        });
    }
    is_ok
}