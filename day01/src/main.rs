use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_instruction(instruction: &String) -> i32 {
    let direction_string = &instruction[0..1];
    let value_string = &instruction[1..];

    let multiplier = if direction_string == "L" { -1 } else { 1 };
    let maybe_parsed_value = value_string.parse::<i32>();
    let parsed_value = if let Ok(value) = maybe_parsed_value {
        value
    } else {
        0
    };

    parsed_value * multiplier
}

pub fn solve_part_one(instructions: Vec<String>) -> u32 {
    let mut current_position: i32 = 50;
    let mut zero_count = 0;

    instructions.iter().for_each(|instruction| {
        println!(
            "Instruction: {}, current position: {}",
            instruction, current_position
        );
        let instruction_value = parse_instruction(instruction);

        current_position = (current_position + instruction_value) % 100;

        if current_position < 0 {
            current_position += 100;
        }

        if current_position == 0 {
            zero_count += 1;
        }

        println!(
            "New position: {}, zero count: {}",
            current_position, zero_count
        );
    });

    zero_count
}

pub fn solve_part_two(instructions: Vec<String>) -> i32 {
    let mut current_position: i32 = 50;
    let mut clicks_count: i32 = 0;

    instructions.iter().for_each(|instruction| {
        println!(
            "Instruction: {}, current position: {}",
            instruction, current_position
        );
        let instruction_value = parse_instruction(instruction);
        let previous_position = current_position;

        let instruction_local_move = instruction_value % 100;

        clicks_count += instruction_value.abs() / 100;

        current_position += instruction_local_move;

        if current_position == 0 {
            if previous_position != 0 {
                clicks_count += 1;
            }
        } else if current_position < 0 {
            current_position += 100;
            if previous_position > 0 {
                clicks_count += 1;
            }
        } else if current_position > 0 {
            if current_position > 99 {
                current_position -= 100;
                clicks_count += 1;
            }
            if previous_position < 0 {
                clicks_count += 1;
            }
        }

        println!(
            "New position: {}, clicks count: {}",
            current_position, clicks_count
        );
    });

    clicks_count
}

fn main() {
    let mut instructions: Vec<String> = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            instructions.push(line);
        }
    }

    println!(
        "Result for part 1 is: {}",
        solve_part_one(instructions.clone())
    );
    println!(
        "Result for part 2 is: {}",
        solve_part_two(instructions.clone())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let instructions = vec![
            String::from("L68"),
            String::from("L30"),
            String::from("R48"),
            String::from("L5"),
            String::from("R60"),
            String::from("L55"),
            String::from("L1"),
            String::from("L99"),
            String::from("R14"),
            String::from("L82"),
        ];
        let result = solve_part_one(instructions);
        assert_eq!(result, 3);
    }

    #[test]
    fn check_solution_part_two() {
        let instructions = vec![
            String::from("L68"),
            String::from("L30"),
            String::from("R48"),
            String::from("L5"),
            String::from("R60"),
            String::from("L55"),
            String::from("L1"),
            String::from("L99"),
            String::from("R14"),
            String::from("L82"),
        ];
        let result = solve_part_two(instructions);
        assert_eq!(result, 6);
    }
}
