use geo::{Contains, LineString, Polygon};

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

#[derive(Clone, Copy)]
pub struct Point {
    x: u64,
    y: u64,
}

fn lines_to_points(lines: &Vec<String>) -> Vec<Point> {
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 2 {
                panic!("Invalid input line: {}", line);
            }
            Point {
                x: parts[0].parse::<u64>().unwrap(),
                y: parts[1].parse::<u64>().unwrap(),
            }
        })
        .collect()
}

pub fn solve_part_one(lines: &Vec<String>) -> u64 {
    let points: Vec<Point> = lines_to_points(lines);

    let mut biggest_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let width = points[j].x.abs_diff(points[i].x) + 1;
            let height = points[j].y.abs_diff(points[i].y) + 1;

            let area = width * height;
            if area > biggest_area {
                println!(
                    "New biggest area found: {} (points: ({},{}) and ({},{})), width: {}, height: {}",
                    area, points[i].x, points[i].y, points[j].x, points[j].y, width, height
                );
                biggest_area = area;
            }
        }
    }

    biggest_area
}

fn lines_to_points_for_line_string(lines: &Vec<String>) -> Vec<(f64, f64)> {
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 2 {
                panic!("Invalid input line: {}", line);
            }
            (
                parts[0].parse::<f64>().unwrap(),
                parts[1].parse::<f64>().unwrap(),
            )
        })
        .collect()
}

pub fn solve_part_two(lines: &Vec<String>) -> f64 {
    let points = lines_to_points_for_line_string(lines);
    let polygon = Polygon::new(LineString::from(points.clone()), vec![]);

    let mut biggest_area = 0.0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let a = points[i];
            let b = points[j];
            let rectangle_points = vec![(a.0, a.1), (a.0, b.1), (b.0, a.1), (b.0, b.1)];
            let rectangle_line_string = LineString::from(rectangle_points.clone());

            // println!(
            //     "Checking rectangle with corners: ({}, {}), ({}, {}), ({}, {}), ({}, {})",
            //     a.0, a.1, a.0, b.1, b.0, a.1, b.0, b.1
            // );

            if !polygon.contains(&rectangle_line_string) {
                // println!("Rectangle is not fully contained in the polygon.");
                continue;
            }

            // println!("Rectangle is fully contained in the polygon.");

            let width = (points[j].0 - points[i].0).abs() + 1.0;
            let height = (points[j].1 - points[i].1).abs() + 1.0;

            let area = width * height;
            if area > biggest_area {
                println!(
                    "New biggest area found: {} (points: ({},{}) and ({},{})), width: {}, height: {}",
                    area, a.0, a.1, b.0, b.1, width, height
                );
                biggest_area = area;
            }
        }
    }

    biggest_area
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
            String::from("7,1"),
            String::from("11,1"),
            String::from("11,7"),
            String::from("9,7"),
            String::from("9,5"),
            String::from("2,5"),
            String::from("2,3"),
            String::from("7,3"),
        ];
        let result = solve_part_one(&lines);
        assert_eq!(result, 50);
    }

    #[test]
    fn check_solution_part_two() {
        let lines = vec![
            String::from("7,1"),
            String::from("11,1"),
            String::from("11,7"),
            String::from("9,7"),
            String::from("9,5"),
            String::from("2,5"),
            String::from("2,3"),
            String::from("7,3"),
        ];
        let result = solve_part_two(&lines);
        assert_eq!(result, 24.0);
    }
}
