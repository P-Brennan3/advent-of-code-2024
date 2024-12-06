use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {

  // part 1
  let file = File::open("inputs/day_1.txt")?;
  let reader = BufReader::new(file);

  let mut right: Vec<i64> = vec![];
  let mut left: Vec<i64> = vec![];

  for line in reader.lines() {
    let line = line?;

    let line: Vec<i64> = line
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();
    
    left.push(line[0]);
    right.push(line[1]);
  }

  left.sort();
  right.sort();

  let mut occurences_in_left_list: HashMap<i64, i64> = HashMap::new();
  let mut total: i64 = 0;
  for i in 0..left.len() {
    total += (left[i] - right[i]).abs();
    *occurences_in_left_list.entry(left[i]).or_insert(0) += 1;
  }

  println!("part 1 {total}");



  // part 2 
  let mut part_2_total = 0;
  for num in right {
    part_2_total += num * occurences_in_left_list.get(&num).unwrap_or(&0);
  }


  println!("part 2 {part_2_total}");


  return Ok(());
}
