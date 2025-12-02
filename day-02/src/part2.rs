#[must_use]
pub fn task(input: &str) -> Option<String> {
    let res: u64 = input
        .split(',')
        // Filter map to avoid explicit unwraps (assume well-formed input)
        .filter_map(|range| {
            let (start, end) = range.split_once('-')?;
            let (start, end) = (start.trim().parse().ok()?, end.trim().parse().ok()?);
            Some(invalid_ids(start, end))
        })
        .sum();
    Some(res.to_string())
}

fn invalid_ids(start: u64, end: u64) -> u64 {
    let mut sum = 0;
    for id in start..=end {
        let s = id.to_string();
        if is_repeating(&s) {
            sum += id;
        }
    }
    sum
}

fn is_repeating(value: &str) -> bool {
    for i in 1..=value.len() / 2 {
        if !value.len().is_multiple_of(i) {
            continue;
        }

        let pattern = &value[..i];
        if pattern.repeat(value.len() / i) == value {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "4174379265");
    }
}
