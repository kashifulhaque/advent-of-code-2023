// Day 1, part 2: https://adventofcode.com/2023/day/1#part2
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{ self, BufRead };

fn main() -> io::Result<()> {
  let mut sum: u32 = 0;
  let file_path = "input.txt";
  let file = File::open(file_path)?;
  let reader = io::BufReader::new(file);
  let number_starts: HashSet<char> = vec!['z', 'o', 't', 'f', 's', 'e', 'n'].into_iter().collect();
  let number_map = HashMap::from([
    ("ze", 0),
    ("on", 1),
    ("tw", 2),
    ("th", 3),
    ("fo", 4),
    ("fi", 5),
    ("si", 6),
    ("se", 7),
    ("ei", 8),
    ("ni", 9)
  ]);
  
  // Read the file, line-by-line
  for line in reader.lines() {
    let line = line?;
    let mut map = HashMap::from([
      ("first", 0),
      ("last", 0)
    ]);

    // Fancy voodoo magic shiz
    for (i, c) in line.char_indices() {
      let num: u32 = c as u32 - 48;

      if num <= 9 {
        if let Some(&value) = map.get("first") {
          if value == 0 {
            map.insert("first", num);
          }
        }
        map.insert("last", num);
      } else {
        if number_starts.contains(&c) && i != (line.chars().count() - 1) {
          let mut first_2_chars = String::from(c);
          first_2_chars.push(line.chars().nth(i + 1).unwrap());
          println!("{}", first_2_chars);
        }
      }
    }
    println!();

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