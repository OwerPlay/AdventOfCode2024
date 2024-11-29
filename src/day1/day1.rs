use crate::utils::utils;

pub fn day1part1() -> String {
    let lines = utils::read_lines("input/day1.txt");
    let mut max_calories = 0;
    let mut sum_calories = 0;
    for line in &lines {
        if line.is_empty() {
            if sum_calories > max_calories {
                max_calories = sum_calories;
            }
            sum_calories = 0;
            continue;
        } else {
            sum_calories += line.parse::<i32>().unwrap();
        }
    }
    String::from(max_calories.to_string())
}

pub fn day1part2() -> String {
    let lines = utils::read_lines("input/day1.txt");
    let mut max_calories = [0i32; 3];
    let mut sum_calories = 0;
    for line in &lines {
        if line.is_empty() {
            if sum_calories > max_calories[0] {
                max_calories[0] = sum_calories;
                max_calories.sort_unstable();
            }
            sum_calories = 0;
        } else {
            sum_calories += line.parse::<i32>().unwrap();
        }
    }
    let total_sum: i32 = max_calories.iter().sum();
    total_sum.to_string()
}