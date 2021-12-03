use std::io::{self, Read};

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_instruction(str: &str) -> Instruction {
    let parts = str.split_whitespace().collect::<Vec<&str>>();
    let direction = parts[0];
    let distance: i32 = parts[1].parse().unwrap();

    if direction == "forward" {
        return Instruction::Forward(distance);
    } else if direction == "down" {
        return Instruction::Down(distance);
    } else if direction == "up" {
        return Instruction::Up(distance);
    }

    panic!("Unknown direction: {}", direction);
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in buffer.lines() {
        instructions.push(parse_instruction(line));
    }

    let (pos_x, pos_y) = apply_instructions(instructions);
    println!("{}", pos_x * pos_y);

    Ok(())
}

fn apply_instructions(instructions: Vec<Instruction>) -> (i32, i32) {
    return instructions
        .iter()
        .fold((0, 0), |(x, y), instruction| match instruction {
            Instruction::Forward(d) => (x + d, y),
            Instruction::Up(d) => (x, y - d),
            Instruction::Down(d) => (x, y + d),
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_add() {
        let (pos_x, pos_y) = apply_instructions(Vec::from(TEST_DATA.map(parse_instruction)));
        assert_eq!(pos_x * pos_y, 150);
    }
}
