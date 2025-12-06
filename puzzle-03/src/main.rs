use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    // load file into reader
    let file_path = Path::new("input.txt");
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    // keep track of total value
    let mut total: u32 = 0;

    // iterate over lines
    for line_result in reader.lines() {
        let line = line_result?;

        // keep track of current first and second numbers and positions
        let mut first: (usize, char) = (0, '0');
        let mut second: (usize, char) = (0, '0');

        // iterate over enumerated characters in string
        for el in line.chars().enumerate() {
            let num = el.1 as u8;

            // compare current element with current highest number
            if num > first.1 as u8 {
                // if not last number in string, make current element first, and reset second
                if el.0 < line.len() - 1 {
                    first = el;
                    second = (0, '0');
                } else {
                    // else use element as second number
                    second = el;
                }
            } else if num > second.1 as u8 {
                // if current element higher than second number, set second number to element
                second = el;
            }
        }

        // concatenate first and second char (number)
        let mut combined = String::new();
        combined.push(first.1);
        combined.push(second.1);

        // increment total by combined number
        total += combined.parse::<u32>().unwrap();
    }

    // print total value
    println!("total: {total}");

    Ok(())
}
