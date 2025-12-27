use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::running_machine::{RunningMachine, solve_running_machine};
use crate::simple_machine::{SimpleMachine, solve_simple_machine};

mod running_machine;
mod simple_machine;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solve_part_one(lines: &Vec<String>) -> u32 {
    let machines: Vec<SimpleMachine> = lines
        .iter()
        .map(|line| SimpleMachine::from_string(line))
        .collect();

    let mut solution = 0;

    machines.iter().for_each(|machine| {
        solution += solve_simple_machine(machine);
    });

    solution
}

pub fn solve_part_two(lines: &Vec<String>) -> usize {
    let machines: Vec<RunningMachine> = lines
        .iter()
        .map(|line| RunningMachine::from_string(line))
        .collect();

    let mut solution = 0;

    machines.iter().enumerate().for_each(|(index, machine)| {
        solution += solve_running_machine(machine, index);
    });

    solution
}

fn main() {
    let mut lines: Vec<String> = Vec::new();

    if let Ok(read_lines) = read_lines("./input.txt") {
        for line in read_lines.map_while(Result::ok) {
            lines.push(line);
        }
    }

    println!("Result for part 1 is: {}", solve_part_one(&lines));
    println!("Result for part 2 is: {}", solve_part_two(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let lines = vec![
            String::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            String::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            String::from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
        ];
        let result = solve_part_one(&lines);
        assert_eq!(result, 7);
    }

    #[test]
    fn check_solution_part_two() {
        let lines = vec![
            String::from("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            String::from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            String::from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
        ];
        let result = solve_part_two(&lines);
        assert_eq!(result, 33);
    }
}
