pub struct RunningMachine {
    buttons: Vec<Vec<u32>>,
    joltage_requirements: Vec<i32>,
}

impl RunningMachine {
    pub fn from_string(line: &String) -> Self {
        let mut parts: Vec<&str> = line.split(' ').collect();
        if parts.len() < 3 {
            panic!("Invalid input string for machine configuration");
        }

        let mut joltage_requirements_string = parts.pop().unwrap();
        joltage_requirements_string =
            &joltage_requirements_string[1..joltage_requirements_string.len() - 1];
        let joltage_requirements: Vec<i32> = joltage_requirements_string
            .split(',')
            .map(|requirement| requirement.parse::<i32>().unwrap())
            .collect();

        let mut buttons: Vec<Vec<u32>> = Vec::new();
        for i in 1..parts.len() {
            let button_values: Vec<u32> = parts[i]
                .chars()
                .filter(|character| {
                    if *character == '(' || *character == ')' || *character == ',' {
                        false
                    } else {
                        true
                    }
                })
                .map(|character| character.to_digit(10).unwrap())
                .collect();

            buttons.push(button_values);
        }

        Self {
            buttons,
            joltage_requirements,
        }
    }
}

fn get_binary_buttons(buttons: &[Vec<u32>]) -> Vec<u32> {
    buttons
        .iter()
        .map(|b| b.iter().map(|n| 1u32 << n).sum())
        .collect()
}

fn subsets<T: Copy>(set: &[T]) -> Vec<Vec<T>> {
    let mut subsets: Vec<Vec<T>> = Vec::new();
    for count in 0..=set.len() {
        subsets.extend(get_combinations(set, count));
    }
    subsets
}

fn get_combinations<T: Copy>(set: &[T], count: usize) -> Vec<Vec<T>> {
    if count == 0 {
        vec![Vec::new()]
    } else {
        set[..set.len() - count + 1]
            .iter()
            .enumerate()
            .flat_map(|(i, &t)| {
                get_combinations(&set[i + 1..], count - 1)
                    .iter()
                    .map(|c| {
                        let mut c1 = c.clone();
                        c1.push(t);
                        c1
                    })
                    .collect::<Vec<Vec<T>>>()
            })
            .collect()
    }
}

// Tranforms the list of joltages into a list of booleans
// where even numbers are false and odd numbers are true
// then converts this to integer for XOR operation
fn get_joltages_as_even_odd_integer(joltages: &[i32]) -> u32 {
    joltages
        .iter()
        .enumerate()
        .map(|(i, j)| ((1 << i) * (j % 2)) as u32)
        .sum()
}

fn solve_running_machine_recursive(
    subset_xors: &[(Vec<u32>, u32)],
    joltages: &[i32],
) -> Option<usize> {
    // If all joltages are 0 then there are no button presses left
    // end recursion
    if joltages.iter().all(|&j| j == 0) {
        return Some(0);
    }

    // Convert joltages to binary value based on whether they are even or odd
    let joltages_as_even_odd_integer = get_joltages_as_even_odd_integer(joltages);

    let mut solution = None;
    for (subset, xor) in subset_xors {
        if !(*xor == joltages_as_even_odd_integer) {
            continue;
        }

        let mut new_joltages = Vec::new();
        let mut mask = 1;
        for &joltage in joltages {
            new_joltages
                .push((joltage - subset.iter().filter(|&b| b & mask != 0).count() as i32) / 2);
            mask <<= 1;
        }

        if new_joltages.iter().all(|&j| j >= 0) {
            let press_count = solve_running_machine_recursive(subset_xors, &new_joltages)
                .map(|c| subset.len() + 2 * c);

            solution = solution.min(press_count).or(solution).or(press_count);
        }
    }
    solution
}

// I have not come up with this solution. I was going for gaussian elimination, but got stuck on handling
// fractions. After a while I gave up. Later I finished the rest of the problems I went to reddit and saw this
// solution here: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// So this is implementation of that solution.
pub fn solve_running_machine(machine: &RunningMachine, index: usize) -> usize {
    println!("Solving machine at index {}", index);

    let binary_buttons = get_binary_buttons(&machine.buttons);
    let subset_xors: Vec<_> = subsets(&binary_buttons)
        .iter()
        .map(|subset| (subset.clone(), subset.iter().fold(0, |a, &b| a ^ b)))
        .collect();

    solve_running_machine_recursive(&subset_xors, &machine.joltage_requirements).unwrap()
}
