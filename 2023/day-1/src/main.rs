use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //finds fist and last digits in a line
    //and calculate all lines

    // part_one();
    part_two();

    //
}

fn part_one() {
    let mut line_total = 0;
    let reader = get_file_content("src/inputs.txt".to_string());
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => continue,
        };

        //get the digits.
        let digits: Vec<char> = line.chars().filter(|ch| ch.is_numeric()).collect();
        print!("---> Digiti---> {:?}", digits);

        //find the first and last digit
        let (first, last) = match (digits.first(), digits.last()) {
            (Some(first), Some(last)) => (first, last),
            _ => continue,
        };

        let num_string: String = format! {"{}{}", first, last};

        if let Ok(num) = num_string.parse::<i32>() {
            line_total += num;
        }
    }

    println!("The answer of the part ONE --> {:?}", line_total);
}

fn part_two() {
    let mut line_total = 0;
    let reader = get_file_content("src/inputs.txt".to_string());
    for line_result in reader.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(_) => continue,
        };

        //get the digits.

        let text_to_num = find_num_in_text(String::from(&line));
        let digits = text_num_to_digit(text_to_num);

        //find the first and last digit
        let (first, last) = match (digits.first(), digits.last()) {
            (Some(first), Some(last)) => (first, last),
            _ => continue,
        };

        let num_string: String = format! {"{}{}", first, last};
        println!("---> Digiti---> {:?} {} {:?}", digits, num_string, line);

        if let Ok(num) = num_string.parse::<i32>() {
            line_total += num;
        }
    }

    println!("The answer of the part TWO --> {:?}", line_total);
}

/// returns content of the give file
fn get_file_content(file_path: String) -> BufReader<File> {
    let file = File::open(file_path).expect("No File-->");
    let reader = BufReader::new(file);
    return reader;
}

fn find_num_in_text(raw_text: String) -> Vec<String> {
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let mut result = vec![];
    // println!("re.iterr--> {:?}", re.find_iter(&raw_text));
    for mat in re.find_iter(&raw_text) {
        // println!(
        //     "{} Found: {} at index {:?}",
        //     raw_text,
        //     mat.as_str(),
        //     mat.start()
        // );
        result.push(mat.as_str().to_string());
    }
    if result.is_empty() {
        result.push("0".to_string())
    }

    result
}

fn text_num_to_digit(str_nums: Vec<String>) -> Vec<i32> {
    let num_look_up = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut result: Vec<i32> = vec![];
    for str_num in str_nums {
        if let Some(&num) = num_look_up.get(str_num.as_str()) {
            result.push(num);
        } else if let Ok(num) = str_num.parse::<i32>() {
            result.push(num);
        }
    }

    return result;
}

fn get_first_and_last_digits(digits: Vec<i32>) -> i32 {
    let (first, last) = match (digits.first(), digits.last()) {
        (Some(first), Some(last)) => (first, last),
        _ => return 0,
    };

    let num_string: String = format! {"{}{}", first, last};
    match num_string.parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

//find text nums in string
#[test]
fn test_find_num_in_text() {
    let test_data: Vec<(&str, Vec<&str>)> = vec![
        ("two1nine", vec!["two", "1", "nine"]),
        ("eightwothree", vec!["eight", "three"]),
        ("abcone2threexyz", vec!["one", "2", "three"]),
        ("xtwone3four", vec!["two", "3", "four"]),
        ("4nineeightseven2", vec!["4", "nine", "eight", "seven", "2"]),
        ("zoneight234", vec!["one", "2", "3", "4"]),
        ("7pqrstsixteen", vec!["7", "six"]),
        ("no digits", vec!["0"]),
    ];

    for data in test_data {
        let test_num = data.0;
        let expected_data = data.1;

        let result = find_num_in_text(String::from(test_num));
        println!(
            "Raw String: {} Expected: {:?} Result {:?}",
            test_num, expected_data, result
        );
        assert_eq!(result, expected_data);
    }
}

/// text num to digits
#[test]
fn test_text_num_to_digit() {
    let text_to_num = find_num_in_text(String::from("sq5fivetwothree1"));

    /// vec!["5", "two", "nine", "two", "1"]
    println!("Result -> {:?}", text_to_num);
    let result = text_num_to_digit(text_to_num);
    println!("Result -> {:?}", result);
    assert_eq!(result, [5, 5, 2, 3, 1]);
}
