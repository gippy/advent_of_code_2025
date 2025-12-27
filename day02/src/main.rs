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

pub struct Range(u64, u64);

fn parse_ranges(ranges: &String) -> Vec<Range> {
    let mut parsed_ranges: Vec<Range> = Vec::new();
    ranges.split(',').for_each(|range| {
        let ids: Vec<&str> = range.split('-').collect();
        if ids.len() != 2 {
            panic!("Range is not in expected format {}", range)
        }
        parsed_ranges.push(Range(
            ids[0].parse::<u64>().unwrap(),
            ids[1].parse::<u64>().unwrap(),
        ));
    });
    parsed_ranges
}

pub fn solve_part_one(ranges: &Vec<Range>) -> u64 {
    let mut solution: u64 = 0;
    ranges.iter().for_each(|range| {
        let mut i = range.0;
        while i <= range.1 {
            let i_as_string = i.to_string();

            // We do not need to check numbers with odd length
            if i_as_string.len() % 2 == 1 {
                i += 1;
                continue;
            }

            // We know this is whole number
            let half_index = i_as_string.len() / 2;
            let first_half = &i_as_string[0..half_index];
            let second_half = &i_as_string[half_index..];
            if first_half == second_half {
                solution += i;
            }
            i += 1;
        }
    });
    solution
}

pub fn solve_part_two(ranges: &Vec<Range>) -> u64 {
    let mut solution: u64 = 0;
    ranges.iter().for_each(|range| {
        println!("Checking range {} - {}", range.0, range.1);
        let mut i = range.0;
        while i <= range.1 {
            let i_as_string = i.to_string();

            let mut is_invalid_id = false;
            let i_length = i_as_string.len();
            'delimiter_loop: for n in 1..(i_length / 2 + 1) {
                // If the string is not divisible by the number of characters into equal chunks we skip the delimiter
                if i_as_string.len() % n != 0 {
                    continue;
                }

                let mut previous_slice = &i_as_string[0..n];
                let mut start = n;
                let mut end = n * 2;
                while end <= i_length {
                    let slice = &i_as_string[start..end];
                    // If we found a slice which does not equal previous slice, we test next delimiter
                    if slice != previous_slice {
                        continue 'delimiter_loop;
                    }
                    previous_slice = slice;
                    start = end;
                    end += n;
                }
                // If we actualy got here, it means that the loop above never encountered different slice, that
                // means the chunks are repeating and we found invalid ID, so we can stop searching for other delimiters
                is_invalid_id = true;
                break 'delimiter_loop;
            }

            if is_invalid_id {
                solution += i;
            }

            i += 1;
        }
    });
    solution
}

fn main() {
    let mut solution_one = 0;
    let mut solution_two = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let parsed_ranges = parse_ranges(&line);
            solution_one += solve_part_one(&parsed_ranges);
            solution_two += solve_part_two(&parsed_ranges);
        }
    }

    println!("Result for part 1 is: {}", solution_one);
    println!("Result for part 2 is: {}", solution_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let input = String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let parsed_ranges = parse_ranges(&input);
        let result = solve_part_one(&parsed_ranges);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn check_solution_part_two() {
        let input = String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let parsed_ranges = parse_ranges(&input);
        let result = solve_part_two(&parsed_ranges);
        assert_eq!(result, 4174379265);
    }
}
