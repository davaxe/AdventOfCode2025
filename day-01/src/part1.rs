#[must_use]
pub fn task(input: &str) -> Option<String> {
    let (_, res) = input
        .lines()
        .filter_map(|line| {
            let dir = line.get(0..1)?;
            let value = line.get(1..)?.parse::<u32>().ok()?;
            Some((dir, value))
        })
        .fold((50u32, 0), |(angle, count), (dir, val)| {
            let angle = match dir {
                "L" => rotate_left(angle, val),
                "R" => rotate_right(angle, val),
                _ => unreachable!("Unexpected direction: {dir}"),
            };
            (angle, count + u32::from(angle == 0))
        });

    Some(res.to_string())
}

fn rotate_left(angle: u32, change: u32) -> u32 {
    let change = change % 100;
    if angle < change {
        100 - (change - angle)
    } else {
        angle - change
    }
}

fn rotate_right(angle: u32, change: u32) -> u32 {
    let change = change % 100;
    let sum = angle + change;
    if sum >= 100 { sum - 100 } else { sum }
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
