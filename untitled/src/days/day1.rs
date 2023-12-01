use crate::file_data;

pub fn calc_calibration_values_sum() {
    let data = file_data("resources/day_1_calibration.txt");

    let to_find = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let calibr_values = data.lines().into_iter()
        .map(|str| {
            let first = str.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = str.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let mut value = String::from(first);
            value.push(last);
            value.parse::<i32>().unwrap()
        })
        .sum::<i32>();

    println!("Calibration values sum: {:?}", calibr_values);
}