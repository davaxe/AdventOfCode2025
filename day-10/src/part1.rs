use std::collections::{HashSet, VecDeque};

use crate::{Machine, NoRequirementMachine, parse};
use chumsky::prelude::*;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let machines = parse::machines().parse(input).into_output()?;
    let res: u32 = machines
        .into_iter()
        .map(Machine::to_no_requirement_machine)
        .filter_map(solve_machine)
        .sum();
    Some(res.to_string())
}

fn solve_machine(machine: NoRequirementMachine) -> Option<u32> {
    let mut queue: VecDeque<(u32, NoRequirementMachine)> = VecDeque::new();
    queue.push_back((0, machine));
    let mut visited_patterns = HashSet::new();

    while let Some((sum, machine)) = queue.pop_front() {
        if machine.is_satisfied() {
            return Some(sum);
        }
        for button in 0..machine.buttons.len() {
            let mut new_machine = machine.clone();
            if new_machine.press_button(button) && !visited_patterns.contains(&new_machine.lights) {
                visited_patterns.insert(new_machine.lights.clone());
                queue.push_back((sum + 1, new_machine));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "7");
    }
}
