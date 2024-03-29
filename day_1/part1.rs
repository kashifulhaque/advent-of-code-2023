// Day 1, part 1: https://adventofcode.com/2023/day/1
use std::fs::File;
use std::collections::HashMap;
use std::io::{ self, BufRead };

fn main() -> io::Result<()> {
  let mut sum: u32 = 0;                   // To keep track of the sum
  let file_path = "input.txt";            // Given input file
  let file = File::open(file_path)?;      // Open the file
  let reader = io::BufReader::new(file);  // Buffer reader (apparently it reads line by line)
  
  // Read the file, line-by-line
  for line in reader.lines() {
    let line = line?;
    let mut map = HashMap::new();         // A hashmap to keep track of the first and last digits of the line
    map.insert("first", 0);
    map.insert("last", 0);

    // Fancy voodoo magic shiz
    for c in line.chars() {
      let num: u32 = c as u32 - 48;

      if num <= 9 {
        if let Some(&value) = map.get("first") {
          if value == 0 {
            map.insert("first", num);
          }
        }

        map.insert("last", num);
      }
    }

    let mut first_num: u32 = 0;
    let mut last_num: u32 = 0;
    if let Some(&value) = map.get("first") { first_num = value; }
    if let Some(&value) = map.get("last") { last_num = value; }
    let final_num = first_num * 10 + last_num;
    sum = sum + final_num;
  }

  println!("Sum: {}", sum);
  Ok(())
}