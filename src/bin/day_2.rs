use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = File::open("inputs/day_2.txt")?;
    let reader = BufReader::new(input);
    
    let mut num_safe = 0;
    let mut num_safe_part_2 = 0;
    for line in reader.lines() {
        let line = line?;
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("parsing error"))
            .collect();
            
        if check_sequence(&nums) {
            num_safe += 1;
        }
        if check_sequence_with_skip(&nums) {
            num_safe_part_2 += 1;
        }
    }
    println!("part 1: {num_safe}");
    println!("part 2: {num_safe_part_2}");
    Ok(())
}

fn check_sequence(nums: &[i32]) -> bool {
    let mut increasing = None;
    
    for i in 0..nums.len()-1 {
        let diff = nums[i].abs_diff(nums[i+1]);
        if diff == 0 || diff > 3 {
            return false;
        }
        
        if increasing.is_none() {
            increasing = Some(nums[i] < nums[i+1]);
            continue;
        }
        
        if (increasing.unwrap() && nums[i] > nums[i+1]) ||
           (!increasing.unwrap() && nums[i] < nums[i+1]) {
            return false;
        }
    }
    true
}

fn check_sequence_with_skip(nums: &[i32]) -> bool {
    // Try the original sequence first
    if check_sequence(nums) {
        return true;
    }
    
    // Try removing each number
    for skip_i in 0..nums.len() {
        let test_nums: Vec<i32> = nums.iter()
            .enumerate()
            .filter(|&(i, _)| i != skip_i)
            .map(|(_, &x)| x)
            .collect();
            
        if check_sequence(&test_nums) {
            return true;
        }
    }
    false
}