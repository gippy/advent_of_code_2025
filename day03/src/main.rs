use std::char;
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

fn line_to_bank(line: &String) -> Vec<u32> {
    let bank: Vec<u32> = line
        .chars()
        .map(|char| {
            if !char.is_numeric() {
                panic!("Character in bank is not a number {}", char);
            }
            char.to_digit(10).unwrap()
        })
        .collect();

    bank
}

pub fn solve_part_one(banks: &Vec<Vec<u32>>) -> u32 {
    let mut solution: u32 = 0;
    banks.iter().for_each(|bank| {
        let mut highest_joltage = 0;
        let mut first_number_index = 0;
        bank.iter().enumerate().for_each(|(i, battery)| {
            if i == (bank.len() - 1) || highest_joltage >= *battery {
                return;
            };
            highest_joltage = *battery;
            first_number_index = i;
        });

        highest_joltage = 0;
        let mut second_number_index = 0;
        bank.iter().enumerate().for_each(|(i, battery)| {
            if i <= first_number_index || highest_joltage >= *battery {
                return;
            };
            highest_joltage = *battery;
            second_number_index = i;
        });

        let joltage = bank[first_number_index] * 10 + bank[second_number_index];

        println!("Joltage {}", joltage);

        solution += joltage;
    });
    solution
}

pub fn solve_part_two(banks: &Vec<Vec<u32>>) -> u64 {
    let mut solution: u64 = 0;

    banks.iter().for_each(|bank| {
        let bank_length = bank.len();

        let mut joltage = String::new();
        let mut previous_highest_index: i32 = -1;
        for n in 0..12 {
            let mut highest_joltage = 0;
            bank.iter().enumerate().for_each(|(i, battery)| {
                let battery_value = *battery;
                let index = i as i32;
                if index <= previous_highest_index
                    || i >= (bank_length - 11 + n)
                    || highest_joltage >= battery_value
                {
                    return;
                };

                highest_joltage = battery_value;
                previous_highest_index = index;
            });
            joltage.push(char::from_digit(highest_joltage, 10).unwrap());
        }

        solution += joltage.parse::<u64>().unwrap();
    });

    solution
}

fn main() {
    let mut banks: Vec<Vec<u32>> = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            banks.push(line_to_bank(&line));
        }
    }

    println!("Result for part 1 is: {}", solve_part_one(&banks));
    println!("Result for part 2 is: {}", solve_part_two(&banks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let banks = vec![
            line_to_bank(&String::from("987654321111111")),
            line_to_bank(&String::from("811111111111119")),
            line_to_bank(&String::from("234234234234278")),
            line_to_bank(&String::from("818181911112111")),
        ];
        let result = solve_part_one(&banks);
        assert_eq!(result, 357);
    }

    #[test]
    fn check_solution_part_two() {
        let banks = vec![
            line_to_bank(&String::from("987654321111111")),
            line_to_bank(&String::from("811111111111119")),
            line_to_bank(&String::from("234234234234278")),
            line_to_bank(&String::from("818181911112111")),
        ];
        let result = solve_part_two(&banks);
        assert_eq!(result, 3121910778619);
    }
}
