use itertools::Itertools;
#[must_use]
pub fn task(input: &str) -> Option<String> {
    let res: u64 = input
        .lines()
        .filter_map(|line| {
            let (x, y) = line.split_once(',')?;
            Some((x.parse::<u64>().ok()?, y.parse::<u64>().ok()?))
        })
        .combinations(2)
        .map(|a| match a.as_slice() {
            [(x1, y1), (x2, y2)] => x1.abs_diff(*x2 - 1) * (y1.abs_diff(*y2 - 1)),
            _ => 0,
        })
        .max()?;

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
        assert_eq!(result.unwrap(), "50");
    }
}
