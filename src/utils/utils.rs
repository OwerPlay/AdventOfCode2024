use std::fmt::Display;
use std::fs;
use std::time::Instant;
fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub(crate) fn read_lines(file_path: &str) -> Vec<String> {
    let contents = read_file(file_path);
    contents
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn benchmark<F, R>(func: F)
where
    F: Fn() -> R,
    R: Display,
{
    let start_time = Instant::now();
    let result = func();
    let duration = start_time.elapsed();
    let elapsed_time = duration.as_secs_f64() * 1000.0;
    println!("Result: {}, Elapsed time: {:.2} ms", result, elapsed_time);
}