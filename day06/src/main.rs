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

fn parse_numbers_line_by_space(line: &String) -> Vec<u64> {
    let parts: Vec<&str> = line.split(" ").collect();

    parts
        .iter()
        .filter(|part| **part != "")
        .map(|part| part.parse::<u64>().unwrap())
        .collect()
}

pub fn solve_part_one(number_lines: &Vec<String>, operations_line: &String) -> u64 {
    let numbers: Vec<Vec<u64>> = number_lines
        .iter()
        .map(|line| parse_numbers_line_by_space(line))
        .collect();

    let operations = parse_operations_line(operations_line);

    let mut solution: u64 = 0;
    let problems_count = operations.len();

    for i in 0..problems_count {
        let mut result: u64 = numbers[0][i];
        let operation = &operations[i];
        for j in 1..numbers.len() {
            match *operation {
                Operation::Multiply => result *= numbers[j][i],
                Operation::Add => result += numbers[j][i],
            }
        }
        solution += result;
    }

    solution
}

pub fn solve_part_two(number_lines: &Vec<String>, operations_line: &String) -> u64 {
    let mut solution: u64 = 0;
    if number_lines.len() == 0 {
        return solution;
    }

    let parsed_number_lines: Vec<Vec<char>> = number_lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let parsed_operations_line: Vec<char> = operations_line.chars().collect();

    let numbers_count = parsed_number_lines[0].len();
    let mut operation = Operation::Add;
    let mut problem_numbers: Vec<u64> = Vec::new();
    for i in 0..numbers_count {
        let mut number_string: String = String::new();
        for j in 0..parsed_number_lines.len() {
            if parsed_number_lines[j][i] == ' ' {
                continue;
            }
            number_string.push(parsed_number_lines[j][i]);
        }
        let number = if number_string.len() != 0 {
            number_string.parse::<u64>().unwrap()
        } else {
            0
        };

        let operation_char = if parsed_operations_line.len() > i {
            parsed_operations_line[i]
        } else {
            ' '
        };

        if operation_char != ' ' {
            if operation_char == '*' {
                operation = Operation::Multiply
            } else {
                operation = Operation::Add
            }
        }

        if number != 0 {
            problem_numbers.push(number);
        } else {
            let result: u64 = match operation {
                Operation::Multiply => problem_numbers
                    .iter()
                    .cloned()
                    .reduce(|acc, e| acc * e)
                    .unwrap_or(1),
                Operation::Add => problem_numbers.iter().sum(),
            };
            solution += result;
            problem_numbers.clear();
        }
    }

    if problem_numbers.len() > 0 {
        let result: u64 = match operation {
            Operation::Multiply => problem_numbers
                .iter()
                .cloned()
                .reduce(|acc, e| acc * e)
                .unwrap_or(1),
            Operation::Add => problem_numbers.iter().sum(),
        };
        solution += result;
    }

    solution
}

pub enum Operation {
    Multiply,
    Add,
}

fn parse_operations_line(line: &String) -> Vec<Operation> {
    let parts: Vec<&str> = line.split(" ").collect();

    parts
        .iter()
        .filter(|part| **part != "")
        .map(|part| {
            if *part == "*" {
                Operation::Multiply
            } else {
                Operation::Add
            }
        })
        .collect()
}

fn main() {
    let mut number_lines: Vec<String> = Vec::new();
    let mut operations_line: String = String::new();

    let mut i = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            if i < 4 {
                number_lines.push(line);
            } else {
                operations_line = line;
            }
            i += 1;
        }
    }

    println!(
        "Result for part 1 is: {}",
        solve_part_one(&number_lines, &operations_line)
    );
    println!(
        "Result for part 2 is: {}",
        solve_part_two(&number_lines, &operations_line)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let number_lines = vec![
            String::from("123 328  51 64 "),
            String::from(" 45 64  387 23 "),
            String::from("  6 98  215 314"),
        ];
        let result = solve_part_one(&number_lines, &String::from("*   +   *   +"));
        assert_eq!(result, 4277556);
    }

    #[test]
    fn check_solution_part_two() {
        let number_lines = vec![
            String::from("123 328  51 64 "),
            String::from(" 45 64  387 23 "),
            String::from("  6 98  215 314"),
        ];
        let result = solve_part_two(&number_lines, &String::from("*   +   *   +"));
        assert_eq!(result, 3263827);
    }
}
