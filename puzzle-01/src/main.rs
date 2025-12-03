use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> io::Result<()> {
    // open the file and load into BufReader
    let file_path = Path::new("rotations.txt");
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    // keep track of times the dial ends up on 0
    let mut count_0: u16 = 0;
    // keep track of the current position of the dial (start at 50 as per AoC description)
    let mut dial_pos: u8 = 50;

    // iterate over lines in reader
    for line_result in reader.lines() {
        let line = line_result?;

        // get direction (first char)
        let direction = line.chars().nth(0).unwrap();
        // get amount to rotate
        let amount: u8 = (line[1..].parse::<u16>().unwrap_or(0) % 100) as u8;
        // check forward or backwards rotation
        if direction == 'L' {
            // if rotating backwards ('L')
            // subtract amount from current dial position (can be negative)
            let new_dial_pos: i8 = dial_pos as i8 - amount as i8;
            // check if passing 0
            if new_dial_pos < 0 {
                // if passing 0, subtract remainder from 100
                dial_pos = 100 - new_dial_pos.abs() as u8;
            } else {
                dial_pos = new_dial_pos as u8;
            }
        } else if direction == 'R' {
            // if rotating forwards ('R')
            // add amount to current dial position
            let new_dial_pos = dial_pos + amount;
            // check if passing 100 (0 on the dial)
            if new_dial_pos > 99 {
                // if passing 100, subtract 100
                dial_pos = new_dial_pos - 100;
            } else {
                dial_pos = new_dial_pos;
            }
        }
        // check if current dial position is 0
        if dial_pos == 0 {
            // if 0, increment count_0
            count_0 += 1;
        }
    }

    println!("Number of times the dial ended up on 0: {count_0}");

    Ok(())
}
