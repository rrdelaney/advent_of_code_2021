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

    let (depth, horizontal, _) = apply_instructions(instructions);
    println!("{}", depth * horizontal);

    Ok(())
}

fn apply_instructions(instructions: Vec<Instruction>) -> (i32, i32, i32) {
    return instructions
        .iter()
        .fold(
            (0, 0, 0),
            |(depth, horizontal, aim), instruction| match instruction {
                Instruction::Forward(d) => (depth + (aim * d), horizontal + d, aim),
                Instruction::Up(d) => (depth, horizontal, aim - d),
                Instruction::Down(d) => (depth, horizontal, aim + d),
            },
        );
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
        let (depth, horizontal, _) =
            apply_instructions(Vec::from(TEST_DATA.map(parse_instruction)));

        assert_eq!(depth * horizontal, 900);
    }
}
