use std::fs;

fn main() {
    let input = fs::read_to_string("day00/input.txt")
        .expect("Failed to read input file");
    
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    // Your solution here
    input.lines().count()
}

fn solve_part2(input: &str) -> usize {
    // Your solution here
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "example\ninput\nhere";
        assert_eq!(solve_part1(input), 3);
    }

    #[test]
    fn test_part2_example() {
        let input = "test";
        assert_eq!(solve_part2(input), 4);
    }
}
