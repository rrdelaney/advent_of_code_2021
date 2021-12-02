use std::io::{self, Read};

fn main() -> io::Result<()> {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer)?;

  let mut data: Vec<i32> = Vec::new();
  for line in buffer.lines() {
    data.push(line.parse().unwrap());
  }

  let incr_count = num_increments_with_window(data);
  println!("{}", incr_count);

  Ok(())
}

fn num_increments_with_window(vals: Vec<i32>) -> i32 {
  let mut vals_shifted_0 = vals.clone();
  let mut vals_shifted_1 = vals_shifted_0.clone();
  vals_shifted_1.remove(0);

  let mut vals_shifted_2 = vals_shifted_1.clone();
  vals_shifted_2.remove(0);

  vals_shifted_1.resize(vals_shifted_1.len() - 1, 0);
  vals_shifted_0.resize(vals_shifted_0.len() - 2, 0);

  let mut incr_count = 0;
  let mut last_seen = i32::MAX;
  for ((a, b), c) in Iterator::zip(Iterator::zip(vals_shifted_0.into_iter(), vals_shifted_1.into_iter()), vals_shifted_2.into_iter()) {
    let iter_val = a + b + c;
    if iter_val > last_seen {
      incr_count += 1;
    }

    last_seen = iter_val;
  }

  return incr_count;
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_DATA: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

  #[test]
  fn test_add() {
    assert_eq!(num_increments_with_window(Vec::from(TEST_DATA)), 5);
  }
}
