use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{AddAssign, MulAssign};

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32
}

impl Position {
    fn new(horizontal: i32, depth: i32) -> Position {
        Position { horizontal, depth }
    }
    fn origin() -> Position {
        Position { horizontal: 0, depth: 0}
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.horizontal += rhs.horizontal;
        self.depth += rhs.depth;
    }
}

impl MulAssign<i32> for Position {
    fn mul_assign(&mut self, rhs: i32) {
        self.horizontal *= rhs;
        self.depth *= rhs;
    }
}

fn main() {
    let filename = "input.txt";
    let instructions = get_parsed_instructions(filename).expect("Expected file is loadable");
    let pos = multiplied_pos_depth(&instructions);
    println!("The multiplied output is: {}", pos);

    println!("Part 2!!!");
    let aimed_pos = with_aim(&instructions);
    println!("Aiming instead, our answer is: {}", aimed_pos);
}

fn instruction_to_offset_vector(command: &str, amount: i32) -> Position {
    let mut unit_vector = match command {
        "forward" => Position { horizontal: 1, depth: 0 },
        "down" => Position { horizontal: 0, depth: 1 },
        "up" => Position { horizontal: 0, depth: -1 },
        _ => unreachable!(format!("Given an unexpected command: {}", command))
    };

    unit_vector *= amount;
    unit_vector
}

fn multiplied_pos_depth(instructions: &[(String, i32)]) -> i32 {
    let mut position = Position::origin();
    for (command, amount) in instructions {
        let offset = instruction_to_offset_vector(&command, *amount);
        position += offset
    }
    position.horizontal * position.depth
}



fn with_aim(instructions: &[(String, i32)]) -> i32 {
    let mut position = Position::origin();
    let mut aim = 0;
    for (command, amount) in instructions {
        match command.as_ref() {
            "down" => aim += amount,
            "up" => aim -= amount,
            "forward" => {
                position.horizontal += amount;
                let to_change = amount * aim;
                position.depth += to_change;
            }
            _ => unreachable!(format!("Given an unexpected command: {}", command))
        }
    }
    position.horizontal * position.depth
}

fn get_parsed_instructions(filename: &str) -> Result<Vec<(String, i32)>, Box<dyn Error>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;

    let mut instructions = vec![];
    for line in lines {
        let mut split = line.split_whitespace().take(2);
        let (command, amount) = (
            split.next().ok_or(format!("Can't read command for line: {}", &line))?,
            split.next().ok_or(format!("Can't read amount for line: {}", &line))?
        );
        let amount: i32 = amount.parse()?;
        instructions.push((command.to_owned(), amount))
    }
    Ok(instructions)
}
