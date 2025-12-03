#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, res) = input
        .lines()
        .filter_map(|line| {
            let dir = line.get(0..1)?;
            let value = line.get(1..)?.parse::<i64>().ok()?;
            Some((dir, value))
        })
        .fold((50u32, 0), |(angle, count), (dir, val)| {
            let angle = match dir {
                "L" => rotate(angle, -val),
                "R" => rotate(angle, val),
                _ => unreachable!("Unexpected direction: {dir}"),
            };
            (angle, count + u32::from(angle == 0))
        });

    Some(res.to_string())
}

#[allow(clippy::cast_possible_truncation)]
fn rotate(angle: u32, change: i64) -> u32 {
    // No truncation as rem_euclid ensures the result is in [0, 99]
    (i64::from(angle) + change).rem_euclid(100) as u32
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
