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
    ("zer", 0),
    ("one", 1),
    ("two", 2),
    ("thr", 3),
    ("fou", 4),
    ("fiv", 5),
    ("six", 6),
    ("sev", 7),
    ("eig", 8),
    ("nin", 9)
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
        if number_starts.contains(&c) && i < (line.chars().count() - 2) {
          let mut first_3_chars = String::from(c);
          first_3_chars.push(line.chars().nth(i + 1).unwrap());
          first_3_chars.push(line.chars().nth(i + 2).unwrap());
          
          if let Some(&value) = number_map.get(first_3_chars.as_str()) {
            if let Some(&val) = map.get("first") {
              if val == 0 {
                map.insert("first", value);
              }
            }

            map.insert("last", value);
          }
        }
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
