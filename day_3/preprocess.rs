use std::fs::File;
use std::collections::HashSet;
use std::io::{ self, BufRead };

fn main() -> io::Result<()> {
  let file_path = "input.txt";
  let file = File::open(file_path)?;
  let reader = io::BufReader::new(file);

  // Make an empty set of 'chars'
  let mut unique_chars_set: HashSet<char> = HashSet::new();

  for line in reader.lines() {
    let line = line?;
    for c in line.chars() {
      unique_chars_set.insert(c);
    }
  }
  for c in &unique_chars_set {
    print!("'{}', ", c);
  }

  println!();
  Ok(())
}
