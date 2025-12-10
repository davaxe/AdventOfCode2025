use crate::{Machine, NoLightsMachine, parse};
use chumsky::prelude::*;
use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem};

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let machines = parse::machines().parse(input).into_output()?;
    let res: f64 = machines
        .into_iter()
        .map(Machine::to_no_lights_machine)
        .filter_map(|machine| solve_machine(&machine))
        .sum();
    Some(res.to_string())
}

fn solve_machine(machine: &NoLightsMachine) -> Option<f64> {
    let x_n = machine.buttons.len();
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let mut vars = vec![];
    for _ in 0..x_n {
        vars.push(problem.add_integer_var(1.0, (0, i32::MAX)));
    }

    for (r, &rhs) in machine.requirements.iter().enumerate() {
        let mut expr = LinearExpr::empty();
        for (c, var) in vars.iter().enumerate() {
            if !machine.buttons[c].connections.contains(&r) {
                continue;
            }
            expr.add(*var, 1.0);
        }
        problem.add_constraint(expr, ComparisonOp::Eq, f64::from(rhs));
    }
    let solution = problem.solve().ok()?;
    Some(solution.objective().round())
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
