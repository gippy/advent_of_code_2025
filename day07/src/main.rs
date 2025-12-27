use std::collections::HashMap;
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

pub fn solve_part_one(lines: &Vec<String>) -> u64 {
    let mut solution: u64 = 0;

    let mut beams: Vec<bool> = lines[0]
        .chars()
        .map(|letter| if letter == 'S' { true } else { false })
        .collect();

    for i in 1..lines.len() {
        lines[i].chars().enumerate().for_each(|(index, letter)| {
            if letter == '.' {
                return;
            }
            if !beams[index] {
                return;
            }

            solution += 1;

            // Split the beam
            beams[index] = false;
            if index > 0 {
                beams[index - 1] = true;
            }
            if index < (beams.len() - 1) {
                beams[index + 1] = true;
            }
        });
    }

    solution
}

fn hash_map_key(beams: &Vec<bool>, row: usize) -> String {
    let mut key = String::new();
    beams.iter().for_each(|is_beam| {
        if *is_beam {
            key.push('|');
        } else {
            key.push('.');
        }
    });
    key.push('-');
    key.push_str(&row.to_string());
    key
}

fn solve_row(
    beams: &Vec<bool>,
    row: usize,
    lines: &Vec<String>,
    solution_counts: &mut HashMap<String, u64>,
) -> u64 {
    let mut solutions: u64 = 0;
    let key = hash_map_key(&beams, row);

    // If we already processed this branch before, just return the already calculated value, it's faster
    match solution_counts.get(&key) {
        Some(count) => {
            return *count;
        }
        None => (),
    }

    // If we are on the last row, there is just one solution left
    if row >= lines.len() {
        solution_counts.insert(key, 1);
        return 1;
    }

    // If we are not on the last row, we need to go through all the options
    lines[row].chars().enumerate().for_each(|(index, letter)| {
        // If beam is not hitting this cell, we do not need to do anything
        if !beams[index] {
            return;
        }

        // If the current cell is not a splitter, we jump to next row
        if letter == '.' {
            solutions += solve_row(beams, row + 1, lines, solution_counts);
            return;
        }

        // Splitter hit

        // Left branch
        if index > 0 {
            let mut left_beams = beams.clone();
            left_beams[index] = false;
            left_beams[index - 1] = true;
            solutions += solve_row(&left_beams, row + 1, lines, solution_counts);
        }

        // Right branch
        if index < (beams.len() - 1) {
            let mut right_beams = beams.clone();
            right_beams[index] = false;
            right_beams[index + 1] = true;
            solutions += solve_row(&right_beams, row + 1, lines, solution_counts);
        }
    });

    let key = hash_map_key(&beams, row);
    solution_counts.insert(key, solutions);
    solutions
}

pub fn solve_part_two(lines: &Vec<String>) -> u64 {
    let mut solution_counts: HashMap<String, u64> = HashMap::new();

    let beams: Vec<bool> = lines[0]
        .chars()
        .map(|letter| if letter == 'S' { true } else { false })
        .collect();

    solve_row(&beams, 1, &lines, &mut solution_counts)
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
            String::from(".......S......."),
            String::from("..............."),
            String::from(".......^......."),
            String::from("..............."),
            String::from("......^.^......"),
            String::from("..............."),
            String::from(".....^.^.^....."),
            String::from("..............."),
            String::from("....^.^...^...."),
            String::from("..............."),
            String::from("...^.^...^.^..."),
            String::from("..............."),
            String::from("..^...^.....^.."),
            String::from("..............."),
            String::from(".^.^.^.^.^...^."),
            String::from("..............."),
        ];
        let result = solve_part_one(&lines);
        assert_eq!(result, 21);
    }

    #[test]
    fn check_solution_part_two() {
        let lines = vec![
            String::from(".......S......."),
            String::from("..............."),
            String::from(".......^......."),
            String::from("..............."),
            String::from("......^.^......"),
            String::from("..............."),
            String::from(".....^.^.^....."),
            String::from("..............."),
            String::from("....^.^...^...."),
            String::from("..............."),
            String::from("...^.^...^.^..."),
            String::from("..............."),
            String::from("..^...^.....^.."),
            String::from("..............."),
            String::from(".^.^.^.^.^...^."),
            String::from("..............."),
        ];
        let result = solve_part_two(&lines);
        assert_eq!(result, 40);
    }
}
