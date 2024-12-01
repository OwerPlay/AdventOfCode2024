use crate::utils::utils;

pub fn day1part1() -> String {
    let lines = utils::read_lines("input/day1.txt");
    let mut left_input: Vec<String> = Vec::new();
    let mut right_input: Vec<String> = Vec::new();

    for line in lines {
        let parts: Vec<String> = line.split("   ").map(|s| s.to_string()).collect();
        left_input.push(parts[0].to_string());
        right_input.push(parts[1].to_string());
    }

    let mut sum = 0;
    left_input.sort_unstable();
    right_input.sort_unstable();

    for i in 0..left_input.len(){
        if left_input[i] > right_input[i] {
            sum += left_input[i].parse::<i32>().unwrap() - right_input[i].parse::<i32>().unwrap();
        } else {
            sum += right_input[i].parse::<i32>().unwrap() - left_input[i].parse::<i32>().unwrap();
        }
    }

    sum.to_string()
}

pub fn day1part2() -> String {
    let lines = utils::read_lines("input/day1.txt");
    let mut left_input: Vec<String> = Vec::new();
    let mut right_input: Vec<String> = Vec::new();
    for line in lines {
        let parts: Vec<String> = line.split("   ").map(|s| s.to_string()).collect();
        left_input.push(parts[0].to_string());
        right_input.push(parts[1].to_string());
    }
    let mut sum = 0;

    for left in left_input {
        let mut count = 0;
        let mut left_number = 0;
        for right in &right_input {
            let left_parsed = left.parse::<i32>();
            match left_parsed {
                Ok(value) => {
                    left_number = value;
                }
                Err(e) => {
                    eprintln!("Failed to parse '{}': {}", left, e);
                }
            }
            let right_parsed = right.parse::<i32>();
            let mut right_number = 0;
            match right_parsed {
                Ok(value) => {
                    right_number = value;
                }
                Err(e) => {
                    eprintln!("Failed to parse '{}': {}", right, e);
                }
            }
            if left_number == right_number {
                count += 1;
            }
        }
        sum += count * left_number;
    }

    sum.to_string()
}