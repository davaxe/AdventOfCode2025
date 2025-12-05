use std::ops::RangeInclusive;

#[must_use]
pub fn task(input: &str) -> Option<String> {
    let mut ranges = input
        .lines()
        .take_while(|l| !l.trim().is_empty())
        .filter_map(|l| {
            let (start, end) = l.split_once('-')?;
            Some(start.parse::<u64>().ok()?..=end.parse::<u64>().ok()?)
        })
        .collect::<Vec<_>>();

    ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()));

    let mut merged = Vec::<RangeInclusive<u64>>::new();
    let mut i = 0;
    while let Some(range) = ranges.get(i) {
        let start = *range.start();
        let mut end = *range.end();

        // Try to extend `end` while following ranges overlap or touch.
        while let Some(next) = ranges.get(i + 1) {
            if *next.start() > end + 1 {
                break;
            }

            end = end.max(*next.end());
            i += 1;
        }

        merged.push(start..=end);
        i += 1;
    }

    let res = merged.iter().map(|r| r.end() - r.start() + 1).sum::<u64>();
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
        assert_eq!(result.unwrap(), "14");
    }
}
