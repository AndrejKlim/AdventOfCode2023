use std::fs;
use crate::days::day1::calc_calibration_values_sum;

mod days;

fn main() {
    calc_calibration_values_sum();
}

pub fn file_data(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}