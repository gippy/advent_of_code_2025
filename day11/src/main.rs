use std::{collections::HashMap, fs};

type NodeName = String;
type Edges = Vec<String>;

#[derive(Clone, Copy, Debug)]
struct PathCounts {
    total_paths: u64,
    to_dac: u64,
    to_fft: u64,
    to_out: u64,
}

fn empty_counts() -> PathCounts {
    PathCounts {
        total_paths: 0,
        to_dac: 0,
        to_fft: 0,
        to_out: 0,
    }
}

fn parse_input(input: &String) -> HashMap<NodeName, Edges> {
    let mut result_hashmap: HashMap<NodeName, Edges> = HashMap::new();

    for line in input.lines() {
        let (key, values) = line.split_once(": ").unwrap();
        for value in values.split(' ') {
            result_hashmap
                .entry(String::from(value))
                .or_default()
                .push(String::from(key));
        }
    }
    result_hashmap
}

fn traverse(
    paths: &HashMap<String, Vec<String>>,
    current_node: &String,
    memory: &mut HashMap<String, PathCounts>,
) -> PathCounts {
    if memory.contains_key(current_node) {
        return *memory.get(current_node).unwrap();
    }

    if !paths.contains_key(current_node) {
        return empty_counts();
    }

    let mut path_counts: PathCounts = paths
        .get(current_node)
        .unwrap()
        .iter()
        .map(|edge_node| traverse(paths, edge_node, memory))
        .fold(empty_counts(), |acc, current_path_counts| PathCounts {
            total_paths: acc.total_paths + current_path_counts.total_paths,
            to_dac: acc.to_dac + current_path_counts.to_dac,
            to_fft: acc.to_fft + current_path_counts.to_fft,
            to_out: acc.to_out + current_path_counts.to_out,
        });

    if current_node == "dac" {
        path_counts.to_out += path_counts.to_fft;
        path_counts.to_dac += path_counts.total_paths;
    }

    if current_node == "fft" {
        path_counts.to_out += path_counts.to_dac;
        path_counts.to_fft += path_counts.total_paths;
    }

    memory.insert(current_node.clone(), path_counts);

    path_counts
}

fn solve_part_one(lines: &String) -> u64 {
    let paths = parse_input(lines);
    let initial_path_counts = PathCounts {
        total_paths: 1,
        to_dac: 0,
        to_fft: 0,
        to_out: 0,
    };
    let mut memory: HashMap<String, PathCounts> =
        HashMap::from([(String::from("you"), initial_path_counts)]);

    let result = traverse(&paths, &String::from("out"), &mut memory);

    result.total_paths
}

fn solve_part_two(lines: &String) -> u64 {
    let paths = parse_input(lines);
    let initial_path_counts = PathCounts {
        total_paths: 1,
        to_dac: 0,
        to_fft: 0,
        to_out: 0,
    };
    let mut memory: HashMap<NodeName, PathCounts> =
        HashMap::from([(String::from("svr"), initial_path_counts)]);

    let result = traverse(&paths, &String::from("out"), &mut memory);

    result.to_out
}

fn main() {
    let lines = fs::read_to_string("input.txt").expect("Cannot find file at input.txt");
    println!("Result for part 1 is: {}", solve_part_one(&lines));
    println!("Result for part 2 is: {}", solve_part_two(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_solution_part_one() {
        let lines = "aaa: you hhh,
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let result = solve_part_one(&String::from(lines));
        assert_eq!(result, 5);
    }

    #[test]
    fn check_solution_part_two() {
        let lines = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let result = solve_part_two(&String::from(lines));
        assert_eq!(result, 2);
    }
}
