
use std::{error::Error, fs::read_to_string};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("inputs/day_3.txt")?;
    
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    let sum: i64 = re.captures_iter(&contents)
    .map(|cap| {        
        // Parse the captured numbers
        let x: i64 = cap[1].parse().unwrap_or(0);
        let y: i64 = cap[2].parse().unwrap_or(0);
        x * y
    })
    .sum();

    println!("part 1: {sum}");
  Ok(())
}