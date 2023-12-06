// Day 2, part 1: https://adventofcode.com/2023/day/2
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
    let cube_cap = HashMap::from([
      ("red", 12),
      ("green", 13),
      ("blue", 14)
    ]);
    
    let tokens: Vec<&str> = line.split(":").collect();

    if let (Some(first_token), Some(last_token)) = (tokens.first(), tokens.last()) {
      // Get the game ID
      let mut game_id: i32 = 0;
      let mut possible = true;
      let game_name_id_tokens = first_token.trim().split(" ");
      if let Some(game_id_token) = game_name_id_tokens.last() { game_id = game_id_token.parse::<i32>().unwrap(); }

      let last_tokens = last_token.trim().split("; ");
      'outer: for token in last_tokens {
        let cube_tokens = token.trim().split(", ");

        for cube_token in cube_tokens {
          let cubes: Vec<&str> = cube_token.trim().split(" ").collect();

          if let (Some(cube_count), Some(cube_colour)) = (cubes.first(), cubes.last()) {
            let cube_count = cube_count.parse::<i32>().unwrap();

            let count = cube_cap.get(cube_colour).unwrap_or(&0);
            if cube_count > *count {
              possible = false;
              break 'outer;
            }

            let count = cube_map.get(cube_colour).unwrap_or(&0);
            cube_map.insert(cube_colour, count + cube_count);
          }
        }
      }

      if possible {
        sum = sum + game_id;
      }
    }
  }

  println!("Sum: {}", sum);
  Ok(())
}
