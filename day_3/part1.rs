use std::fs::File;
use std::collections::HashSet;
use std::io::{ self, BufRead };

fn main() -> io::Result<()> {
  let special_chars: HashSet<char> = vec!['*', '%', '+', '@', '&', '#', '$', '-', '=', '/'].into_iter().collect();
  let file_path = "input.txt";
  let file = File::open(file_path)?;
  let reader = io::BufReader::new(file);

  Ok(())
}
