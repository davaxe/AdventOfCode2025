#[derive(Debug, Clone, Copy)]
enum Value {
    Num(u64),
    Op(Operator),
}

impl Value {
    fn to_num(self) -> Option<u64> {
        match self {
            Self::Num(n) => Some(n),
            Self::Op(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let values: Vec<Vec<Value>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| {
                    v.parse::<u64>().map_or_else(
                        |_| match v {
                            "+" => Value::Op(Operator::Add),
                            "*" => Value::Op(Operator::Multiply),
                            _ => unreachable!("unknown operator"),
                        },
                        Value::Num,
                    )
                })
                .collect()
        })
        .collect();

    // Transpose the values
    let transposed: Vec<Vec<Value>> = (0..values[0].len())
        .map(|i| values.iter().map(|row| row[i]).collect())
        .collect();

    let mut res: u64 = 0;
    for value in transposed {
        if let Some(Value::Op(op)) = value.last() {
            res += match op {
                Operator::Add => value.into_iter().filter_map(Value::to_num).sum::<u64>(),
                Operator::Multiply => value.into_iter().filter_map(Value::to_num).product(),
            }
        }
    }

    Some(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "4277556");
    }
}
