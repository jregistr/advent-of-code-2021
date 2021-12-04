use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let instructions = get_instructions("input.txt")
        .expect("Expected that instructions file can be loaded.");

    let cloned = instructions.clone();

    print_power_consumption(&instructions, 12);

    println!("---- Part 2 ----");
    let oxygen = get_oxygen_rating(cloned, 12);
    let scrubber = get_scrubber_rating(instructions.clone(), 12);
    println!("Answer is: {}", oxygen * scrubber);
}

fn get_oxygen_rating(mut instructions: Vec<String>, bits_length: usize) -> i32 {
    for index in 0..bits_length {
        let (ones_count, zeroes_count) = count_ones_and_zeroes(&instructions, bits_length);
        let a = ones_count[index];
        let b = zeroes_count[index];

        if a >= b {
            let mut inst_dex = 0;
            while instructions.len() > 1 && inst_dex < instructions.len() {
                let at = &instructions[inst_dex];
                if at.chars().nth(index).unwrap() == '0' {
                    instructions.remove(inst_dex);
                } else {
                    inst_dex += 1;
                }
            }
        } else {
            let mut inst_dex = 0;
            while instructions.len() > 1 && inst_dex < instructions.len() {
                let at = &instructions[inst_dex];
                if at.chars().nth(index).unwrap() == '1' {
                    instructions.remove(inst_dex);
                } else {
                    inst_dex += 1;
                }
            }
        }

        if instructions.len() == 1 {
            break;
        }
    }

    let first = instructions.first().unwrap();
    i32::from_str_radix(first, 2).unwrap()
}

fn get_scrubber_rating(mut instructions: Vec<String>, bits_length: usize) -> i32 {
    for index in 0..bits_length {
        let (ones_count, zeroes_count) = count_ones_and_zeroes(&instructions, bits_length);
        let a = ones_count[index];
        let b = zeroes_count[index];

        if a < b {
            let mut inst_dex = 0;
            while instructions.len() > 1 && inst_dex < instructions.len() {
                let at = &instructions[inst_dex];
                if at.chars().nth(index).unwrap() == '0' {
                    instructions.remove(inst_dex);
                } else {
                    inst_dex += 1;
                }
            }
        } else {
            let mut inst_dex = 0;
            while instructions.len() > 1 && inst_dex < instructions.len() {
                let at = &instructions[inst_dex];
                if at.chars().nth(index).unwrap() == '1' {
                    instructions.remove(inst_dex);
                } else {
                    inst_dex += 1;
                }
            }
        }

        if instructions.len() == 1 {
            break;
        }
    }

    let first = instructions.first().unwrap();
    i32::from_str_radix(first, 2).unwrap()
}

fn print_power_consumption(instructions: &[String], bits_length: usize) {
    let (ones_count, zeroes_count) = count_ones_and_zeroes(instructions, bits_length);

    let gamma = summarize(&ones_count, &zeroes_count, |a, b| a > b);
    let epsilon = summarize(&ones_count, &zeroes_count, |a, b| a < b);

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();
    println!("The answer is: {}", gamma * epsilon);
}

fn count_ones_and_zeroes(instructions: &[String], bits_length: usize) -> (Vec<i32>, Vec<i32>) {
    let mut ones_count = vec![0; bits_length];
    let mut zeroes_count = vec![0; bits_length];

    for instruction in instructions {
        let instruction = i32::from_str_radix(&instruction, 2).unwrap();
        for i in (0..bits_length).rev() {

            let mask = 1 << i;
            let has_one = instruction & mask != 0;
            let ar_index = (bits_length - 1) - i;
            // println!("Ar Index: {}", ar_index);
            if has_one {
                ones_count[ar_index] += 1;
            } else {
                zeroes_count[ar_index] += 1;
            }
        }
    }
    (ones_count, zeroes_count)
}

fn summarize(ones_count: &[i32], zeroes_count: &[i32], put_one: fn(&i32, &i32) -> bool) -> String {
    ones_count.iter().zip(zeroes_count.iter())
        .map(|(a, b)| if put_one(a, b) { '1' } else { '0' })
        .collect::<String>()
}

fn get_instructions(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let as_vec = reader.lines()
        .map(|maybe_line| maybe_line.unwrap())
        .collect::<Vec<String>>();
    Ok(as_vec)
}
