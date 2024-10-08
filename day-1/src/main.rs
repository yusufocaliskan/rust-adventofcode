use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    //finds fist and last digits in a line
    //and calculate all lines
    part_one();

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
        print!("{:?}", digits);

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

    let alphabetic_nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let reader = get_file_content("src/inputs.txt".to_string());

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            //get the digits.
            let digits: Vec<char> = line.chars().filter(|ch| ch.is_numeric()).collect();
            // print!("{:?}", digits);

            //find the first and last digit
            if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
                //format them
                let num_string: String = format! {"{}{}", first, last};
                if let Ok(num) = num_string.parse::<i32>() {
                    line_total += num;
                }
            }
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
