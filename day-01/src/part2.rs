#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, res) = input
        .lines()
        .filter_map(|line| {
            let dir = line.get(0..1)?;
            let value = line.get(1..)?.parse::<u32>().ok()?;
            Some((dir, value))
        })
        .fold((50, 0), |(angle, count), (dir, val)| {
            let (angle, zeros) = match dir {
                "L" => rotate_left(angle, val),
                "R" => rotate_right(angle, val),
                _ => unreachable!("Unexpected direction: {dir}"),
            };
            (angle, count + zeros)
        });

    Some(res.to_string())
}

fn rotate_left(angle: u32, change: u32) -> (u32, u32) {
    let wraps = change / 100;
    let rem = change % 100;
    let new_angle = (angle + 100 - rem) % 100;
    let crossings = u32::from(angle != 0 && rem >= angle) + wraps;
    (new_angle, crossings)
}

fn rotate_right(angle: u32, change: u32) -> (u32, u32) {
    let wraps = change / 100;
    let rem = change % 100;
    let new_angle = (angle + rem) % 100;
    let crossings = u32::from(angle + rem >= 100) + wraps;
    (new_angle, crossings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "6");
    }
}
