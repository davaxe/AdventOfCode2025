use crate::{Machine, NoLightsMachine, parse};
use chumsky::prelude::*;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let machines = parse::machines().parse(input).into_output()?;
    let res: u32 = machines
        .into_iter()
        .map(Machine::to_no_lights_machine)
        .filter_map(solve_machine)
        .sum();
    Some(res.to_string())
}

fn solve_machine(machine: NoLightsMachine) -> Option<u32> {
    todo!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "33");
    }
}
