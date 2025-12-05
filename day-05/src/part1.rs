#[must_use]
pub fn task(input: &str) -> Option<String> {
    let ranges = input
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .filter_map(|l| {
            let (start, end) = l.split_once('-')?;
            Some(start.parse::<u64>().ok()?..=end.parse::<u64>().ok()?)
        })
        .collect::<Vec<_>>();
    let res = input
        .lines()
        .skip(ranges.len())
        .filter_map(|n| {
            let n = n.parse::<u64>().ok()?;
            ranges.iter().position(|r| r.contains(&n))
        })
        .count();

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
        assert_eq!(result.unwrap(), "3");
    }
}
