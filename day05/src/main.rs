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

pub fn solve_part_one(ranges: &Vec<Range>, ingredient_ids: &Vec<u64>) -> u32 {
    let mut solution: u32 = 0;

    ingredient_ids.iter().for_each(|ingredient_id| {
        let mut is_fresh = false;
        ranges.iter().for_each(|range| {
            if is_fresh {
                return;
            }
            if *ingredient_id >= range.from && *ingredient_id <= range.to {
                is_fresh = true
            }
        });
        if is_fresh {
            solution += 1;
        }
    });

    solution
}

pub fn solve_part_two(ranges: &Vec<Range>) -> u64 {
    let mut solution: u64 = 0;

    let mut ranges_to_process: Vec<Range> = ranges.clone();
    while ranges_to_process.len() != 0 {
        let maybe_current_range = ranges_to_process.pop();
        let Some(current_range) = maybe_current_range else {
            break;
        };

        let mut merged = false;
        ranges_to_process = ranges_to_process
            .iter()
            .map(|range| {
                if current_range.from > range.to {
                    return *range;
                }
                if current_range.to < range.from {
                    return *range;
                }

                merged = true;

                let mut new_range = *range;

                if current_range.from < range.from {
                    new_range.from = current_range.from;
                }

                if current_range.to > range.to {
                    new_range.to = current_range.to;
                }

                new_range
            })
            .collect();

        if !merged {
            // println!(
            //     "Adding range: {}-{} = {}",
            //     current_range.from,
            //     current_range.to,
            //     current_range.to - current_range.from + 1
            // );
            solution += current_range.to - current_range.from + 1;
        }
    }

    solution
}

#[derive(Clone, Copy)]
pub struct Range {
    from: u64,
    to: u64,
}

fn parse_range_line(line: &String) -> Range {
    let parts: Vec<&str> = line.split('-').collect();
    if parts.len() < 2 {
        panic!("Invalid range line: {}", line);
    }

    Range {
        from: parts[0].parse::<u64>().unwrap(),
        to: parts[1].parse::<u64>().unwrap(),
    }
}

fn parse_ingredient_id_line(line: &String) -> u64 {
    line.parse::<u64>().unwrap()
}

fn main() {
    let mut ranges: Vec<Range> = Vec::new();
    let mut ingredient_ids: Vec<u64> = Vec::new();
    let mut switch_conversion = false;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            if line == "" {
                switch_conversion = true;
                continue;
            }

            if switch_conversion {
                ingredient_ids.push(parse_ingredient_id_line(&line));
            } else {
                ranges.push(parse_range_line(&line));
            }
        }
    }

    println!(
        "Result for part 1 is: {}",
        solve_part_one(&ranges, &ingredient_ids)
    );
    println!("Result for part 2 is: {}", solve_part_two(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let ranges = vec![
            parse_range_line(&String::from("3-5")),
            parse_range_line(&String::from("10-14")),
            parse_range_line(&String::from("16-20")),
            parse_range_line(&String::from("12-18")),
        ];
        let ingredient_ids = vec![
            parse_ingredient_id_line(&String::from("1")),
            parse_ingredient_id_line(&String::from("5")),
            parse_ingredient_id_line(&String::from("8")),
            parse_ingredient_id_line(&String::from("11")),
            parse_ingredient_id_line(&String::from("17")),
            parse_ingredient_id_line(&String::from("32")),
        ];
        let result = solve_part_one(&ranges, &ingredient_ids);
        assert_eq!(result, 3);
    }

    #[test]
    fn check_solution_part_two() {
        let ranges = vec![
            parse_range_line(&String::from("3-5")),
            parse_range_line(&String::from("10-14")),
            parse_range_line(&String::from("16-20")),
            parse_range_line(&String::from("12-18")),
        ];
        let result = solve_part_two(&ranges);
        assert_eq!(result, 14);
    }
}
