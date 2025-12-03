#[must_use]
pub fn task(input: &str) -> Option<String> {
    let res: u32 = input
        .lines()
        .map(|line| {
            max_jolts(
                line.chars()
                    .map(|c| u32::from(c) - '0' as u32)
                    .collect::<Vec<u32>>()
                    .as_slice(),
            )
        })
        .sum();

    Some(res.to_string())
}

fn max_jolts(bank: &[u32]) -> u32 {
    let mut max = 0;
    for (left_i, left) in bank.iter().enumerate() {
        for right in bank.iter().skip(left_i + 1) {
            let value = left * 10 + right;
            if value > max {
                max = value;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "357");
    }
}
