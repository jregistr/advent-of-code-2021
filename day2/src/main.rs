use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{AddAssign, MulAssign};

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: i32,
    y: i32
}

impl Vector {
    fn origin() -> Vector {
        Vector {x: 0, y: 0}
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl MulAssign<i32> for Vector {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

fn instruct_to_vector(inst: &str) -> Vector {
    let (command, amount) = instruct_line_to_tuple(inst);
    let commd: &str = command.as_ref();

    let mut vector = match commd {
        "forward" => Vector {x: 1, y: 0},
        "down" => Vector {x: 0, y: -1},
        "up" => Vector {x: 0, y: 1},
        _ => unreachable!("This shouldn't happen")
    };

    vector *= amount;
    vector
}

fn instruct_line_to_tuple(inst: &str) -> (String, i32) {
    let mut split = inst.split_whitespace().take(2);
    let (command, amount) = (split.next().unwrap(), split.next().unwrap());
    let amount = amount.parse().unwrap();
    (command.to_owned(), amount)
}

fn main() {
    let filename = "input.txt";
    let instructions = get_file(filename).expect("Expected file is loadable");
    print_multiplied_pos(&instructions);

    println!("Part 2!!!");
    with_aim(&instructions);
}

fn with_aim(instructions: &[String]) {
    let mut position = Vector::origin();
    let mut aim = 0;

    for line in instructions {
        let (command, amount) = instruct_line_to_tuple(line);
        let command: &str = command.as_ref();

        match command  {
            "down" => aim += amount,
            "up" => aim -= amount,
            "forward" => {
                position.x += amount;
                let tochang = amount * aim;
                position.y += tochang;
            }
            _ => unreachable!("No")
        }
    }

    let mult = (position.x * position.y).abs();
    println!("The multiplied output is: {}", mult);
}

fn print_multiplied_pos(instructions: &[String]) {
    let mut position = Vector::origin();

    for line in instructions {
        let update = instruct_to_vector(&line);
        position += update;
    }
    let mult = (position.x * position.y).abs();
    println!("The multiplied output is: {}", mult);
}

fn get_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f).lines();
    Ok(reader.flat_map(|item| item.ok()).collect())
}