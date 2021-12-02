use std::io::{self, Read};

fn main() -> io::Result<()> {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer)?;

  let mut data: Vec<i32> = Vec::new();
  for line in buffer.lines() {
    data.push(line.parse().unwrap());
  }

  let incr_count = num_increments(data);
  println!("{}", incr_count);

  Ok(())
}

fn num_increments(vals: Vec<i32>) -> i32 {
  let mut incr_count = 0;
  let mut last_seen = vals[0];
  for elem in vals {
    if elem > last_seen {
      incr_count += 1;
    }

    last_seen = elem;
  }

  return incr_count;
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_DATA: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

  #[test]
  fn test_add() {
    assert_eq!(num_increments(Vec::from(TEST_DATA)), 7);
  }
}
