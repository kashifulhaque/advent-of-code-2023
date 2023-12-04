// Day 1, part 2: https://adventofcode.com/2023/day/1#part2
use std::collections::HashSet;

fn main() {
  let number_starts: HashSet<str> = vec!['z', 'o', 't', 'f', 's', 'e', 'n'].into_iter().collect();
  println!("{:?}", number_starts);
}