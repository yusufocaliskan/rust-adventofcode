use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/inputs.txt").expect("No File-->");
    let reader = BufReader::new(file);
    let mut line_total = 0;
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            //get the digits.
            let digits: Vec<char> = line.chars().filter(|ch| ch.is_numeric()).collect();
            print!("{:?}", digits);

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

    println!("Right Answer Is =-> {:?}", line_total);
    Ok(())
}
