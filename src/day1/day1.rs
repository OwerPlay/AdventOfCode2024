use crate::utils::utils;

pub fn day1part1() -> String {
    let lines = utils::read_lines("input/day1.txt");
    let (mut left_input, mut right_input) = lines_to_u32(lines);

    let mut sum = 0;
    left_input.sort_unstable();
    right_input.sort_unstable();

    left_input.iter().zip(&right_input).for_each(|(left, right)| {
        sum += left.abs_diff(*right);
    });

    sum.to_string()
}

pub fn day1part2() -> String {
    let lines = utils::read_lines("input/day1.txt");
    let (left_input, right_input) = lines_to_u32(lines);

    let mut sum = 0;

    left_input.iter().for_each(|left| {
        let mut count = 0;
        right_input.iter().for_each(|right| {
            if left == right {
                count += 1;
            }
        });
        sum += count * left;
    });

    sum.to_string()
}

fn lines_to_u32(lines: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut left_input: Vec<u32> = Vec::new();
    let mut right_input: Vec<u32> = Vec::new();

    lines.iter().for_each(|line| {
        let parts: Vec<Result<u32, _>> = line.split_whitespace().map(|s| s.parse()).collect();
        if let (Some(Ok(left)), Some(Ok(right))) = (parts.get(0), parts.get(1)) {
            left_input.push(*left);
            right_input.push(*right);
        } else {
            eprintln!("Failed to parse line: {}", line);
        }
    });

    (left_input, right_input)
}