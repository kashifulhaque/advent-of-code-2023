// Day 2, part 2: https://adventofcode.com/2023/day/2#part2
use std::cmp;
use std::fs::File;
use std::collections::HashMap;
use std::io::{ self, BufRead };

fn main() -> io::Result<()> {
  let mut sum = 0;
  let file_path = "input.txt";
  let file = File::open(file_path)?;
  let reader = io::BufReader::new(file);

  for line in reader.lines() {
    let line = line?;
    let mut cube_map = HashMap::from([
      ("red", 0),
      ("green", 0),
      ("blue", 0)
    ]);
    
    let tokens: Vec<&str> = line.split(":").collect();

    if let Some(last_token) = tokens.last() {
      let last_tokens = last_token.trim().split("; ");

      // Iterate over tokens of a line (after the ':' part)
      for token in last_tokens {
        let cube_tokens = token.trim().split(", ");

        for cube_token in cube_tokens {
          let cubes: Vec<&str> = cube_token.trim().split(" ").collect();

          if let (Some(cube_count), Some(cube_colour)) = (cubes.first(), cubes.last()) {
            let cube_count = cube_count.parse::<i32>().unwrap();
            let count = cube_map.get(cube_colour).unwrap_or(&0);
            cube_map.insert(cube_colour, cmp::max(*count, cube_count));
          }
        }
      }

      let mut power = 1;
      for (_colour, count) in cube_map.clone().into_iter() {
        power = power * count;
      }
      sum = sum + power;
    }
  }

  println!("Sum: {}", sum);
  Ok(())
}
