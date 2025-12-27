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

fn convert_line(line: &String) -> Vec<bool> {
    line.chars()
        .map(|char| if char == '@' { true } else { false })
        .collect()
}

pub fn solve_part_one(map: &Vec<Vec<bool>>) -> u32 {
    let mut solution: u32 = 0;
    let map_size = map.len();
    map.iter().enumerate().for_each(|(row_index, row)| {
        let previous_row_exists = row_index > 0;
        let next_row_exists = row_index < (map_size - 1);

        row.iter().enumerate().for_each(|(cell_index, is_roll)| {
            if !is_roll {
                return;
            };
            let previous_cell_exists = cell_index > 0;
            let next_cell_exists = cell_index < (map_size - 1);

            let mut adjacent_roll_count = 0;
            adjacent_roll_count += if previous_row_exists
                && previous_cell_exists
                && map[row_index - 1][cell_index - 1]
            {
                1
            } else {
                0
            };
            adjacent_roll_count += if previous_row_exists && map[row_index - 1][cell_index] {
                1
            } else {
                0
            };
            adjacent_roll_count +=
                if previous_row_exists && next_cell_exists && map[row_index - 1][cell_index + 1] {
                    1
                } else {
                    0
                };
            adjacent_roll_count += if previous_cell_exists && map[row_index][cell_index - 1] {
                1
            } else {
                0
            };
            adjacent_roll_count += if next_cell_exists && map[row_index][cell_index + 1] {
                1
            } else {
                0
            };
            adjacent_roll_count +=
                if next_row_exists && previous_cell_exists && map[row_index + 1][cell_index - 1] {
                    1
                } else {
                    0
                };
            adjacent_roll_count += if next_row_exists && map[row_index + 1][cell_index] {
                1
            } else {
                0
            };
            adjacent_roll_count +=
                if next_row_exists && next_cell_exists && map[row_index + 1][cell_index + 1] {
                    1
                } else {
                    0
                };
            if adjacent_roll_count < 4 {
                solution += 1;
            }
        });
    });

    solution
}

pub fn solve_part_two(map: &Vec<Vec<bool>>) -> u32 {
    let mut solution: u32 = 0;
    let map_size = map.len();
    let mut check_again = true;
    let mut work_in_progress_map = map.clone();
    while check_again {
        let mut removed_rolls = 0;
        work_in_progress_map = work_in_progress_map
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                let previous_row_exists = row_index > 0;
                let next_row_exists = row_index < (map_size - 1);

                row.iter()
                    .enumerate()
                    .map(|(cell_index, is_roll)| {
                        if !is_roll {
                            return *is_roll;
                        };
                        let previous_cell_exists = cell_index > 0;
                        let next_cell_exists = cell_index < (map_size - 1);

                        let mut adjacent_roll_count = 0;
                        adjacent_roll_count += if previous_row_exists
                            && previous_cell_exists
                            && work_in_progress_map[row_index - 1][cell_index - 1]
                        {
                            1
                        } else {
                            0
                        };
                        adjacent_roll_count += if previous_row_exists
                            && work_in_progress_map[row_index - 1][cell_index]
                        {
                            1
                        } else {
                            0
                        };
                        adjacent_roll_count += if previous_row_exists
                            && next_cell_exists
                            && work_in_progress_map[row_index - 1][cell_index + 1]
                        {
                            1
                        } else {
                            0
                        };
                        adjacent_roll_count += if previous_cell_exists
                            && work_in_progress_map[row_index][cell_index - 1]
                        {
                            1
                        } else {
                            0
                        };
                        adjacent_roll_count += if next_cell_exists
                            && work_in_progress_map[row_index][cell_index + 1]
                        {
                            1
                        } else {
                            0
                        };
                        adjacent_roll_count += if next_row_exists
                            && previous_cell_exists
                            && work_in_progress_map[row_index + 1][cell_index - 1]
                        {
                            1
                        } else {
                            0
                        };
                        adjacent_roll_count +=
                            if next_row_exists && work_in_progress_map[row_index + 1][cell_index] {
                                1
                            } else {
                                0
                            };
                        adjacent_roll_count += if next_row_exists
                            && next_cell_exists
                            && work_in_progress_map[row_index + 1][cell_index + 1]
                        {
                            1
                        } else {
                            0
                        };

                        if adjacent_roll_count < 4 {
                            removed_rolls += 1;
                            return false;
                        }

                        true
                    })
                    .collect()
            })
            .collect();

        if removed_rolls == 0 {
            check_again = false;
        }

        solution += removed_rolls;
    }

    solution
}

fn main() {
    let mut map: Vec<Vec<bool>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            map.push(convert_line(&line));
        }
    }

    println!("Result for part 1 is: {}", solve_part_one(&map));
    println!("Result for part 2 is: {}", solve_part_two(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let map: Vec<Vec<bool>> = vec![
            convert_line(&String::from("..@@.@@@@.")),
            convert_line(&String::from("@@@.@.@.@@")),
            convert_line(&String::from("@@@@@.@.@@")),
            convert_line(&String::from("@.@@@@..@.")),
            convert_line(&String::from("@@.@@@@.@@")),
            convert_line(&String::from(".@@@@@@@.@")),
            convert_line(&String::from(".@.@.@.@@@")),
            convert_line(&String::from("@.@@@.@@@@")),
            convert_line(&String::from(".@@@@@@@@.")),
            convert_line(&String::from("@.@.@@@.@.")),
        ];
        let result = solve_part_one(&map);
        assert_eq!(result, 13);
    }

    #[test]
    fn check_solution_part_two() {
        let map: Vec<Vec<bool>> = vec![
            convert_line(&String::from("..@@.@@@@.")),
            convert_line(&String::from("@@@.@.@.@@")),
            convert_line(&String::from("@@@@@.@.@@")),
            convert_line(&String::from("@.@@@@..@.")),
            convert_line(&String::from("@@.@@@@.@@")),
            convert_line(&String::from(".@@@@@@@.@")),
            convert_line(&String::from(".@.@.@.@@@")),
            convert_line(&String::from("@.@@@.@@@@")),
            convert_line(&String::from(".@@@@@@@@.")),
            convert_line(&String::from("@.@.@@@.@.")),
        ];
        let result = solve_part_two(&map);
        assert_eq!(result, 43);
    }
}
