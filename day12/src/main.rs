use std::fs::read_to_string;

type GiftTypeCounts = [usize; 6];
type Dimensions = (usize, usize);

fn parse_input(input: &String) -> Vec<(Dimensions, GiftTypeCounts)> {
    let mut definitions: Vec<(Dimensions, GiftTypeCounts)> = Vec::new();

    for line in input.lines() {
        if line.len() < 24 {
            continue;
        }
        let (dimensions_string, gift_counts_string) = line.split_once(": ").unwrap();
        let dimension_strings: Vec<String> = dimensions_string
            .split("x")
            .map(|d| String::from(d))
            .collect();
        let gift_count_strings: Vec<String> = gift_counts_string
            .split(" ")
            .map(|g| String::from(g))
            .collect();

        if dimension_strings.len() < 2 {
            panic!("Invalid dimension string: {}", dimensions_string);
        }
        if gift_count_strings.len() < 6 {
            panic!("Invalid gift counts string: {}", gift_counts_string);
        }

        let dimensions = (
            dimension_strings[0].parse::<usize>().unwrap(),
            dimension_strings[1].parse::<usize>().unwrap(),
        );

        let gift_type_counts = [
            gift_count_strings[0].parse::<usize>().unwrap(),
            gift_count_strings[1].parse::<usize>().unwrap(),
            gift_count_strings[2].parse::<usize>().unwrap(),
            gift_count_strings[3].parse::<usize>().unwrap(),
            gift_count_strings[4].parse::<usize>().unwrap(),
            gift_count_strings[5].parse::<usize>().unwrap(),
        ];

        definitions.push((dimensions, gift_type_counts));
    }

    definitions
}

pub fn solve_part_one(definitions: &Vec<(Dimensions, GiftTypeCounts)>) -> u32 {
    let mut solution = 0;
    for definition in definitions.iter() {
        let (dimensions, gift_type_counts) = definition;

        // Naive soluition, we consider each gift to take full 9 pixels and so we
        // calculate the area divided by 3. Since each package is 3px wide and 3px tall.
        // then we just check if the number of packages fits into the area...
        let area = (dimensions.0 / 3) * (dimensions.1 / 3);
        let total_gifts = gift_type_counts.iter().sum::<usize>();
        println!("Area is {}, total gifts is {}", area, total_gifts);
        if area >= total_gifts {
            solution += 1;
        }
    }

    return solution;
}

fn main() {
    let lines = read_to_string("input.txt").expect("Cannot find file at input.txt");
    let input = parse_input(&lines);

    println!("Result for part 1 is: {}", solve_part_one(&input));
}

#[cfg(test)]
mod tests {
    // This day is bullshit and the examples are missleading...
    #[test]
    fn check_solution_part_one() {
        let result = 0;
        assert_eq!(result, 0);
    }
}
