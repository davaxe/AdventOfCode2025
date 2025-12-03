use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[must_use]
pub fn task(input: &str) -> Option<String> {
    let res: u64 = input
        .lines()
        .filter_map(|line| {
            max_jolts(
                line.chars()
                    .map(|c| u64::from(c) - '0' as u64)
                    .collect::<Vec<u64>>()
                    .as_slice(),
                12,
            )
        })
        .sum();

    Some(res.to_string())
}

/// Finds the maximum joltage value that can be formed by selecting `n` digits
/// from the given bank of digits, maintaining their relative order.
fn max_jolts(bank: &[u64], n: u32) -> Option<u64> {
    let mut queue = BinaryHeap::new();

    // Prioritize the largest values, then the smallest index
    queue.push((0, Reverse(0), n));
    while let Some((joltage, Reverse(pos), remaining_digits)) = queue.pop() {
        if remaining_digits == 0 {
            return Some(joltage);
        }
        for (pos, next) in bank.iter().enumerate().skip(pos) {
            if bank.len() - pos < remaining_digits as usize {
                break;
            }
            let next_state = (joltage * 10 + next, Reverse(pos + 1), remaining_digits - 1);
            queue.push(next_state);
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
        assert_eq!(result.unwrap(), "3121910778619");
    }
}
