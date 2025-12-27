use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64;
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

pub struct Box {
    x: u64,
    y: u64,
    z: u64,
}

pub struct Connection {
    from: usize,
    to: usize,
    distance: f64,
}

fn lines_to_boxes(lines: &Vec<String>) -> Vec<Box> {
    lines
        .iter()
        .map(|line| {
            let parts: Vec<u64> = line
                .split(",")
                .map(|part| part.parse::<u64>().unwrap())
                .collect();

            if parts.len() < 3 {
                panic!("Invalid input line: {}", line);
            }
            Box {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

fn euclidean_distance(a: &Box, b: &Box) -> f64 {
    let mut distance_squared: f64 = 0.0;
    distance_squared += ((a.x as f64) - (b.x as f64)).powf(2.0);
    distance_squared += ((a.y as f64) - (b.y as f64)).powf(2.0);
    distance_squared += ((a.z as f64) - (b.z as f64)).powf(2.0);
    distance_squared.sqrt()
}

fn calculate_distances(boxes: &Vec<Box>) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            connections.push(Connection {
                from: i,
                to: j,
                distance: euclidean_distance(&boxes[i], &boxes[j]),
            });
        }
    }
    connections.sort_by(|a, b| {
        b.distance
            .partial_cmp(&a.distance)
            .unwrap_or(Ordering::Equal)
    });
    connections
}

fn add_connection_to_circuits(
    connection: &Connection,
    circuits: &Vec<HashSet<usize>>,
) -> Vec<HashSet<usize>> {
    let mut circuit_found = false;

    let mut new_circuits: Vec<HashSet<usize>> = Vec::new();

    let mut new_merge_circuit_index = usize::MAX;
    let mut circuits_to_merge: Vec<HashSet<usize>> = Vec::new();

    circuits.iter().for_each(|circuit| {
        let mut from_found = false;
        let mut to_found = false;
        circuit.iter().for_each(|box_id| {
            if *box_id == connection.from {
                from_found = true;
            }
            if *box_id == connection.to {
                to_found = true;
            }
        });

        if from_found || to_found {
            // If we already found another circuit with same connection, we have added the circuit there,
            // so for additional ciruits including the same connection we will need those to be merged
            // which is why we are not adding them to the new circuits array.
            if circuit_found {
                circuits_to_merge.push(circuit.clone());
                return;
            }
            circuit_found = true;
            new_merge_circuit_index = new_circuits.len();
        }

        let mut new_circuit = circuit.clone();

        if !from_found && !to_found {
            new_circuits.push(new_circuit);
            return;
        }

        new_circuit.insert(connection.to);
        new_circuit.insert(connection.from);

        new_circuits.push(new_circuit);
    });

    // We have not found any curcuit for the connection, add it as new circuit
    if !circuit_found {
        new_circuits.push(HashSet::from([connection.from, connection.to]));
    } else if circuits_to_merge.len() > 0 {
        circuits_to_merge.iter().for_each(|circuit| {
            circuit.iter().for_each(|box_id| {
                new_circuits[new_merge_circuit_index].insert(*box_id);
            });
        });
    }

    new_circuits
}

pub fn solve_part_one(lines: &Vec<String>, max_operations: usize) -> u64 {
    let boxes = lines_to_boxes(lines);
    let mut possible_connections = calculate_distances(&boxes);
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for _i in 0..max_operations {
        let Some(shortest_connection) = possible_connections.pop() else {
            break;
        };
        circuits = add_connection_to_circuits(&shortest_connection, &circuits);
    }

    if circuits.len() < 3 {
        return 0;
    };

    let mut circuit_lengts: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    circuit_lengts.sort();

    let mut solution: u64 = 1;
    for _i in 0..3 {
        if let Some(length) = circuit_lengts.pop() {
            solution *= length as u64;
        };
    }

    solution
}

pub fn solve_part_two(lines: &Vec<String>) -> u64 {
    let boxes = lines_to_boxes(lines);
    let mut possible_connections = calculate_distances(&boxes);
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    let mut solution = 0;
    while circuits.len() != 1 || circuits[0].len() != boxes.len() {
        let Some(shortest_connection) = possible_connections.pop() else {
            break;
        };
        circuits = add_connection_to_circuits(&shortest_connection, &circuits);
        solution = boxes[shortest_connection.from].x * boxes[shortest_connection.to].x;
    }

    solution
}

fn main() {
    let mut lines: Vec<String> = Vec::new();

    if let Ok(read_lines) = read_lines("./input.txt") {
        for line in read_lines.map_while(Result::ok) {
            lines.push(line);
        }
    }

    println!("Result for part 1 is: {}", solve_part_one(&lines, 1000));
    println!("Result for part 2 is: {}", solve_part_two(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let lines = vec![
            String::from("162,817,812"),
            String::from("57,618,57"),
            String::from("906,360,560"),
            String::from("592,479,940"),
            String::from("352,342,300"),
            String::from("466,668,158"),
            String::from("542,29,236"),
            String::from("431,825,988"),
            String::from("739,650,466"),
            String::from("52,470,668"),
            String::from("216,146,977"),
            String::from("819,987,18"),
            String::from("117,168,530"),
            String::from("805,96,715"),
            String::from("346,949,466"),
            String::from("970,615,88"),
            String::from("941,993,340"),
            String::from("862,61,35"),
            String::from("984,92,344"),
            String::from("425,690,689"),
        ];
        let result = solve_part_one(&lines, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn check_solution_part_two() {
        let lines = vec![
            String::from("162,817,812"),
            String::from("57,618,57"),
            String::from("906,360,560"),
            String::from("592,479,940"),
            String::from("352,342,300"),
            String::from("466,668,158"),
            String::from("542,29,236"),
            String::from("431,825,988"),
            String::from("739,650,466"),
            String::from("52,470,668"),
            String::from("216,146,977"),
            String::from("819,987,18"),
            String::from("117,168,530"),
            String::from("805,96,715"),
            String::from("346,949,466"),
            String::from("970,615,88"),
            String::from("941,993,340"),
            String::from("862,61,35"),
            String::from("984,92,344"),
            String::from("425,690,689"),
        ];
        let result = solve_part_two(&lines);
        assert_eq!(result, 25272);
    }
}
