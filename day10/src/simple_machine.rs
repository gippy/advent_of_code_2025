use queues::*;

use std::collections::HashMap;

pub struct SimpleMachine {
    expected_state: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    _joltage_requirements: Vec<u32>,
}

impl SimpleMachine {
    pub fn from_string(line: &String) -> Self {
        let mut parts: Vec<&str> = line.split(' ').collect();
        if parts.len() < 3 {
            panic!("Invalid input string for machine configuration");
        }

        let mut joltage_requirements_string = parts.pop().unwrap();
        joltage_requirements_string =
            &joltage_requirements_string[1..joltage_requirements_string.len() - 1];
        let joltage_requirements: Vec<u32> = joltage_requirements_string
            .split(',')
            .map(|requirement| requirement.parse::<u32>().unwrap())
            .collect();

        let expected_state = parts[0]
            .chars()
            .filter(|character| {
                if *character == '[' || *character == ']' {
                    false
                } else {
                    true
                }
            })
            .map(|character| if character == '#' { true } else { false })
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
            expected_state,
            buttons,
            _joltage_requirements: joltage_requirements,
        }
    }
}

fn hashmap_key(state: &Vec<bool>) -> String {
    state
        .iter()
        .map(|is_active| if *is_active { '#' } else { '.' })
        .collect::<String>()
}

fn check_state(expected_state: &Vec<bool>, current_state: &Vec<bool>) -> bool {
    if expected_state.len() != current_state.len() {
        panic!("State length mismatch");
    }
    let mut is_equal = true;
    for i in 0..expected_state.len() {
        if !is_equal {
            continue;
        }
        if expected_state[i] != current_state[i] {
            is_equal = false;
        }
    }
    is_equal
}

pub fn solve_simple_machine(machine: &SimpleMachine) -> u32 {
    let initial_state: Vec<bool> = machine.expected_state.iter().map(|_| false).collect();

    let mut memory: HashMap<String, u32> = HashMap::new();

    let mut queue: Queue<(Vec<bool>, u32)> = Queue::new();
    let Ok(_result) = queue.add((initial_state, 0)) else {
        panic!("Failed to add initial state to queue");
    };

    let mut minimum_presses = u32::MAX;
    while queue.size() > 0 {
        let Ok(item) = queue.remove() else {
            break;
        };

        let (state, presses) = item;

        let key = hashmap_key(&state);
        if memory.contains_key(&key) && *memory.get(&key).unwrap() <= presses {
            continue;
        }

        memory.insert(hashmap_key(&state), presses);

        // Early exit if we already exceeded shortest found distance
        if presses >= minimum_presses {
            continue;
        }

        if check_state(&machine.expected_state, &state) {
            if presses < minimum_presses {
                minimum_presses = presses;
            }
            continue;
        }

        let new_presses = presses + 1;
        machine.buttons.iter().for_each(|button| {
            let mut new_state = state.clone();
            button.iter().for_each(|&position| {
                let index = position as usize;
                if index >= new_state.len() {
                    return;
                }
                new_state[index] = !new_state[index];
            });

            let new_key = hashmap_key(&new_state);
            if !memory.contains_key(&new_key) || *memory.get(&new_key).unwrap() >= new_presses {
                queue.add((new_state, new_presses)).unwrap();
            }
        });
    }
    minimum_presses
}
