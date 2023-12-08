use std::fs;
use std::path::PathBuf;

use crate::days::day_base::DayTrait;

pub struct Day {}

impl DayTrait for Day {
    fn get_id_str() -> String {
        "day_01".to_string()
    }
    fn get_result_str() -> String {
        // Load file TODO: Move to common
        let read_path: PathBuf =
            fs::canonicalize("./src/days/day_01/input").expect("Error converting path.");
        let value: String = fs::read_to_string(read_path.as_path()).expect("Error reading.");
        // Process string
        return format!(
            "{}\n{}",
            Day::process_file_str(&value, false),
            Day::process_file_str(&value, true)
        );
    }
}

impl Day {
    fn process_digits(str_value: &str, include_number_strings: bool) -> u32 {
        let mut first_digit: Option<u32> = None;
        let mut first_digit_idx: Option<usize> = None;
        let mut last_digit: Option<u32> = None;
        let mut last_digit_idx: Option<usize> = None;
        for (i, c) in str_value.chars().enumerate() {
            let c_result = c.to_digit(10);
            if c_result.is_some() {
                if first_digit.is_none() {
                    first_digit = c_result;
                    first_digit_idx = Some(i);
                }
                last_digit = c_result;
                last_digit_idx = Some(i);
            }
        }

        if include_number_strings {
            let nums_as_str = [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            for (i, s) in nums_as_str.into_iter().enumerate() {
                let result_fwd = str_value.find(s);
                let result_rvs = str_value.rfind(s);
                if result_fwd.is_some() {
                    if first_digit_idx.is_none() || first_digit_idx.unwrap() > result_fwd.unwrap()
                    {
                        first_digit = Some(i as u32);
                        first_digit_idx = result_fwd;
                    }
                    if last_digit_idx.is_none() || last_digit_idx.unwrap() < result_rvs.unwrap() {
                        last_digit = Some(i as u32);
                        last_digit_idx = result_rvs;
                    }
                }
            }
        }

        assert!(
            first_digit.is_some(),
            "First digit of {} at least must be a non null value.",
            str_value
        );
        if first_digit.is_none() {
            return 0;
        } else {
            // Bit scuff, but str composition is useless here.
            // Also wtf is a turbofish?
            return 10 * first_digit.unwrap() + last_digit.unwrap();
        }
    }

    pub fn process_file_str(str_value: &str, include_number_strings: bool) -> String {
        let string_parts = str_value.split("\n");
        let mut sum: u32 = 0;
        for part in string_parts {
            let trimmed_part = part.trim();
            if trimmed_part == "" {
                continue;
            }
            let num = Day::process_digits(trimmed_part, include_number_strings);
            sum += num;
        }
        return sum.to_string();
    }
}

#[cfg(test)]
mod day_01_test {
    // Needed because otherwise the module doesn't know about
    // the day struct even though it's in the same f***ing file.
    use crate::days::day_01::day_impl_01::Day;
    #[test]
    fn test_with_base_str() {
        let test_str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let out_value = Day::process_file_str(&test_str, false);
        assert_eq!(out_value, String::from("142"));
    }
    #[test]
    fn test_with_base_str_str_interp() {
        let test_str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n";
        let out_value = Day::process_file_str(&test_str, true);
        assert_eq!(out_value, String::from("281"));
    }
}
