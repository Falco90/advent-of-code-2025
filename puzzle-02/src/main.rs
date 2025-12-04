use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // create string from file
    let file_path = Path::new("input.txt");
    let string = fs::read_to_string(file_path)?;
    // split the file into ranges
    let ranges: Vec<&str> = string.split(',').collect();

    // keep track of the total value of invalid IDs
    let mut total_value: u128 = 0;

    // for each range, separate lower and higher bound
    for range in ranges {
        let bounds: Vec<u128> = range
            .split('-')
            .map(|el| el.trim().parse::<u128>().unwrap())
            .collect();

        // for each number between bound, check first and second half
        for n in bounds[0]..bounds[1] {
            // convert number to string
            let string = n.to_string();
            // get first half
            let first_half: &str = &string[..string.len() / 2];
            // get second half
            let second_half: &str = &string[string.len() / 2..];

            // compare first and second half
            if first_half == second_half {
                // increment total value if halves are equal
                total_value += n;
            }
        }
    }

    // print the total value of invalid IDs
    println!("total value: {total_value}");

    Ok(())
}
