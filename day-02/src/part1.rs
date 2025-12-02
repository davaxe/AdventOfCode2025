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

#[allow(clippy::cast_possible_truncation)]
fn invalid_ids(start: u64, end: u64) -> u64 {
    let mut sum = 0;
    let mut id = start;
    while id <= end {
        let s = id.to_string();
        if !s.len().is_multiple_of(2) {
            id = 10u64.pow(s.len() as u32);
            continue;
        }
        let half = s.len() / 2;
        if s[..half] == s[half..] {
            sum += id;
        }
        id += 1;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "1227775554");
    }
}
